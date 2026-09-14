#!/usr/bin/env python3
"""
Generate coverage data and test statistics for a Rust workspace project with historical tracking.
Cross-platform script for Windows and Linux.

Usage:
    python scripts/generate-coverage.py --project <project-name> [options]

Options:
    --project, -p    Project name to generate coverage for (required)
    --docs-dir       Custom docs directory (default: docs/<project>)
    --no-html        Skip HTML coverage report generation
    --no-history     Skip history tracking
"""

import argparse
import json
import os
import re
import sys
import subprocess
import shutil
import glob
from pathlib import Path
from datetime import datetime, timezone

# Default configuration
DEFAULT_RETENTION_DAYS_FULL = 30
DEFAULT_RETENTION_DAYS_WEEKLY = 180
DEFAULT_RETENTION_DAYS_MONTHLY = 365


def parse_args():
    """Parse command line arguments."""
    parser = argparse.ArgumentParser(
        description="Generate coverage data and test statistics for a Rust workspace project."
    )
    parser.add_argument(
        "--project", "-p",
        required=True,
        help="Project name to generate coverage for"
    )
    parser.add_argument(
        "--docs-dir",
        default=None,
        help="Custom docs directory (default: docs/<package>)"
    )
    parser.add_argument(
        "--no-html",
        action="store_true",
        help="Skip HTML coverage report generation"
    )
    parser.add_argument(
        "--no-history",
        action="store_true",
        help="Skip history tracking"
    )
    return parser.parse_args()


def get_project_info(package_name, args):
    """Get project configuration based on package name."""
    docs_dir = args.docs_dir if args.docs_dir else f"docs/{package_name}"
    coverage_dir = Path(docs_dir) / "coverage"
    data_dir = Path("docs/assets/data") / package_name
    history_file = data_dir / "coverage-history.json"
    coverage_file = data_dir / "coverage.json"
    stats_file = data_dir / "stats.json"
    
    return {
        "package": package_name,
        "docs_dir": docs_dir,
        "coverage_dir": coverage_dir,
        "data_dir": data_dir,
        "history_file": history_file,
        "coverage_file": coverage_file,
        "stats_file": stats_file,
    }


def get_git_info():
    """Get current git commit information."""
    try:
        commit_sha = subprocess.run(
            ["git", "rev-parse", "HEAD"],
            capture_output=True,
            text=True,
            check=True
        ).stdout.strip()
        
        commit_msg = subprocess.run(
            ["git", "log", "-1", "--pretty=%B"],
            capture_output=True,
            text=True,
            check=True
        ).stdout.strip().split('\n')[0]
        
        timestamp = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
        
        return commit_sha, commit_msg, timestamp
    except subprocess.CalledProcessError:
        return "unknown", "No git information available", datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def load_history(history_file):
    """Load existing coverage history or create new."""
    if history_file.exists():
        try:
            with open(history_file, 'r') as f:
                return json.load(f)
        except (json.JSONDecodeError, IOError) as e:
            print(f"Warning: Failed to load history, starting fresh: {e}", file=sys.stderr)
    
    return {
        "version": "1.0",
        "last_updated": "",
        "snapshots": [],
        "coverage_summary": {}
    }


def apply_retention_policy(snapshots, retention_days_full=30, retention_days_weekly=180, retention_days_monthly=365):
    """Apply retention policy to limit history size.
    
    Keeps the most recent snapshot from each time period:
    - All snapshots from last 30 days
    - One snapshot per week for days 31-180 (most recent in each week)
    - One snapshot per month for days 181-365 (most recent in each month)
    - One snapshot per quarter beyond 365 days (most recent in each quarter)
    """
    if not snapshots:
        return [], {"removed": 0, "kept": 0}
    
    now = datetime.now(timezone.utc)
    original_count = len(snapshots)
    
    parsed_snapshots = []
    for snapshot in snapshots:
        try:
            timestamp_str = snapshot['timestamp'].replace('Z', '+00:00')
            timestamp = datetime.fromisoformat(timestamp_str)
            age_days = (now - timestamp).days
            parsed_snapshots.append((timestamp, age_days, snapshot))
        except (ValueError, KeyError) as e:
            print(f"Warning: Skipping malformed snapshot: {e}", file=sys.stderr)
            continue
    
    parsed_snapshots.sort(key=lambda x: x[0], reverse=True)
    
    retained = []
    seen_weeks = set()
    seen_months = set()
    seen_quarters = set()
    
    retention_stats = {
        "full": 0,
        "weekly": 0,
        "monthly": 0,
        "quarterly": 0
    }
    
    for timestamp, age_days, snapshot in parsed_snapshots:
        if age_days <= retention_days_full:
            retained.append(snapshot)
            retention_stats["full"] += 1
        elif age_days <= retention_days_weekly:
            week_key = (timestamp.year, timestamp.isocalendar()[1])
            if week_key not in seen_weeks:
                retained.append(snapshot)
                seen_weeks.add(week_key)
                retention_stats["weekly"] += 1
        elif age_days <= retention_days_monthly:
            month_key = (timestamp.year, timestamp.month)
            if month_key not in seen_months:
                retained.append(snapshot)
                seen_months.add(month_key)
                retention_stats["monthly"] += 1
        else:
            quarter = (timestamp.month - 1) // 3 + 1
            quarter_key = (timestamp.year, quarter)
            if quarter_key not in seen_quarters:
                retained.append(snapshot)
                seen_quarters.add(quarter_key)
                retention_stats["quarterly"] += 1
    
    retained_sorted = sorted(retained, key=lambda s: s['timestamp'])
    
    summary = {
        "removed": original_count - len(retained_sorted),
        "kept": len(retained_sorted),
        "breakdown": retention_stats
    }
    
    return retained_sorted, summary


def normalize_coverage_filenames(coverage_file, package_name="vb6parse"):
    """Normalize filenames in coverage.json from absolute paths to relative paths.
    
    Converts paths like:
        /home/arthur/.../vb6/projects/vb6parse/src/errors/mod.rs
    To:
        errors/mod.rs
    """
    try:
        with open(coverage_file, 'r') as f:
            coverage = json.load(f)
        
        prefix = 'projects/' + package_name + '/src/'
        
        for file_entry in coverage.get('data', [{}])[0].get('files', []):
            old_path = file_entry.get('filename', '')
            idx = old_path.rfind(prefix)
            if idx != -1:
                file_entry['filename'] = old_path[idx + len(prefix):]
        
        with open(coverage_file, 'w') as f:
            json.dump(coverage, f)
        
        return True
    except (IOError, KeyError, IndexError) as e:
        print(f"Warning: Failed to normalize coverage filenames: {e}", file=sys.stderr)
        return False


def run_coverage(package_info):
    """Run cargo llvm-cov to generate coverage data.
    
    Temporarily removes cdylib from crate-type because cargo-llvm-cov sets
    CARGO_TARGET_DIR internally which breaks cdylib hardlinking.
    Uses an isolated target directory to prevent caching artifacts built
    without coverage instrumentation.
    """
    print("Generating coverage data...")
    
    coverage_file = package_info["coverage_file"]
    package = package_info["package"]
    
    coverage_file.parent.mkdir(parents=True, exist_ok=True)
    
    # Find the package's Cargo.toml
    possible_paths = [
        Path(f"projects/{package}/Cargo.toml"),
        Path(f"{package}/Cargo.toml"),
        Path("Cargo.toml"),
    ]
    cargo_toml = None
    for path in possible_paths:
        if path.exists():
            content = path.read_text()
            if f'name = "{package}"' in content:
                cargo_toml = path
                break
    
    if cargo_toml is None:
        raise RuntimeError(f"Could not find Cargo.toml for package '{package}'")
    
    original = cargo_toml.read_text()
    env = os.environ.copy()
    cov_target = Path.cwd() / "target" / "llvm-cov"
    cov_target.mkdir(parents=True, exist_ok=True)
    # Clean isolated target to avoid stale non-instrumented artifacts
    for child in cov_target.iterdir():
        if child.is_dir():
            shutil.rmtree(str(child))
        else:
            child.unlink()
    env["CARGO_LLVM_COV_TARGET_DIR"] = str(cov_target)
    
    try:
        modified = original.replace(
            'crate-type = ["cdylib", "rlib"]',
            'crate-type = ["rlib"]'
        )
        if modified == original:
            raise RuntimeError("Could not modify crate-type")
        cargo_toml.write_text(modified)
        
        try:
            subprocess.run(
                ["cargo", "llvm-cov", "--package", package, "--lib", "--tests", "--json", "--output-path", str(coverage_file)],
                check=True,
                env=env
            )
            normalize_coverage_filenames(coverage_file, package)
        finally:
            cargo_toml.write_text(original)
    except subprocess.CalledProcessError as e:
        print(f"Warning: cargo llvm-cov failed ({e})", file=sys.stderr)
        if not coverage_file.exists():
            print("No existing coverage.json found. Exiting.", file=sys.stderr)
            sys.exit(1)
        print(f"Using existing coverage.json: {coverage_file}")


def generate_html_coverage(package_info, project_root):
    """Generate HTML coverage reports using llvm-cov."""
    print("Generating HTML coverage reports...")
    
    coverage_dir = package_info["coverage_dir"]
    package = package_info["package"]
    
    coverage_dir.mkdir(parents=True, exist_ok=True)
    
    # Generate JSON coverage first
    run_coverage(package_info)
    
    # Find the package's Cargo.toml
    possible_paths = [
        Path(f"projects/{package}/Cargo.toml"),
        Path(f"{package}/Cargo.toml"),
        Path("Cargo.toml"),
    ]
    cargo_toml = None
    for path in possible_paths:
        if path.exists():
            content = path.read_text()
            if f'name = "{package}"' in content:
                cargo_toml = path
                break
    
    if cargo_toml is None:
        raise RuntimeError(f"Could not find Cargo.toml for package '{package}'")
    
    original = cargo_toml.read_text()
    env = os.environ.copy()
    cov_target = Path.cwd() / "target" / "llvm-cov"
    cov_target.mkdir(parents=True, exist_ok=True)
    for child in cov_target.iterdir():
        if child.is_dir():
            shutil.rmtree(str(child))
        else:
            child.unlink()
    env["CARGO_LLVM_COV_TARGET_DIR"] = str(cov_target)
    
    try:
        modified = original.replace(
            'crate-type = ["cdylib", "rlib"]',
            'crate-type = ["rlib"]'
        )
        if modified == original:
            raise RuntimeError("Could not modify crate-type")
        cargo_toml.write_text(modified)
        
        try:
            subprocess.run(
                [
                    "cargo", 
                    "llvm-cov", 
                    "--package", package,
                    "--lib",
                    "--tests",
                    "--html",
                    "--output-dir", 
                    str(coverage_dir)
                ],
                check=True,
                env=env
            )
            
            restructure_coverage_html(coverage_dir, package, project_root)
            
            print(f"✓ HTML coverage reports generated in {coverage_dir}")
            return coverage_dir
        finally:
            cargo_toml.write_text(original)
    except subprocess.CalledProcessError as e:
        print(f"Warning: cargo llvm-cov --html failed ({e})", file=sys.stderr)
    
    if coverage_dir.exists():
        html_files = list(coverage_dir.rglob('*.html'))
        if html_files:
            print(f"Reprocessing {len(html_files)} existing HTML files...")
            fix_coverage_html_paths(coverage_dir, package, project_root)
        else:
            print("Warning: No existing HTML files found in coverage/", file=sys.stderr)
    else:
        print("Warning: No existing coverage/ directory found", file=sys.stderr)
    
    return coverage_dir


def fix_coverage_html_paths(coverage_dir, package_name, project_root):
    """Fix paths in HTML files in the coverage directory."""
    import re
    
    pattern = re.compile(r"(href|src)='coverage/[^']*?/src/", re.IGNORECASE)
    replacement = r"\1='"
    abs_pattern = re.compile(r"(href|src)='[^']*?/projects/" + re.escape(package_name) + r"/src/", re.IGNORECASE)
    source_title_pattern = re.compile(
        r"<div class='source-name-title'><pre>([^<]+)</pre></div>",
        re.IGNORECASE
    )
    
    def source_title_replacement(match):
        full_path = match.group(1)
        prefix = '/' + package_name + '/src/'
        if prefix in full_path:
            relative_path = full_path.split(prefix)[-1]
        else:
            relative_path = full_path    
        title = match.group(1)
        return f"<div class='source-name-title'><a href='https://github.com/scriptandcompile/vb6/tree/master/projects/{package_name}/src/{relative_path}'>{relative_path}</a></div>"
    
    files_processed = 0
    for html_file in coverage_dir.rglob('*.html'):
        content = html_file.read_text(encoding='utf-8')
        
        relative_to_cov = html_file.relative_to(coverage_dir)
        depth_from_cov = len(relative_to_cov.parts) - 1
        
        css_depth = depth_from_cov + 3
        css_base = '../' * css_depth + 'assets/css/'
        control_path = '../' * css_depth + 'assets/js/coverage/control.js'
        js_path = '../' * css_depth + 'assets/js/'
        
        css_link = f"<link rel='stylesheet' type='text/css' href='{css_base}llvm-cov.css'>"
        style_link = f"<link rel='stylesheet' type='text/css' href='{css_base}style.css'>"
        theme_script_src = f'<script src="{js_path}theme-switcher.js"></script>'
        
        css_links = re.findall(r"<link rel='stylesheet' type='text/css' href='[^']*\.css'>", content)
        if len(css_links) >= 1:
            content = content.replace(css_links[0], css_link, 1)
        if len(css_links) >= 2:
            content = content.replace(css_links[1], style_link, 1)
        if 'style.css' not in content and len(css_links) == 1:
            content = content.replace(css_link, css_link + '\n' + style_link)
        
        control_matches = re.findall(r"""src=['"][^'"]*control\.js['"]""", content)
        for match in control_matches:
            content = content.replace(match, f"src='{control_path}'", 1)
        
        theme_matches = re.findall(r"<script src=[\"'][^\"']*theme-switcher\.js[\"']></script>", content)
        for match in theme_matches:
            content = content.replace(match, theme_script_src, 1)
        if 'theme-switcher.js' not in content:
            content = content.replace('<body>', '<body>\n' + theme_script_src)
        
        content = re.sub(r"</script>\s*<span class='control'[^>]*>.*?</span>", "</script>", content, flags=re.DOTALL)
        
        nav_up_depth = depth_from_cov + 1
        docs_path = '../' * nav_up_depth + 'index.html'
        coverage_page_path = '../' * nav_up_depth + 'coverage.html'
        
        has_custom_header = f"{package_name.title()} Coverage Report</h1>" in content or f"{package_name} Coverage Report</h1>" in content
        
        if not has_custom_header:
            header_html = f"""<header>
    <div class="container">
        <h1>{package_name.title()} Coverage Report</h1>
        <p class="tagline">Generated from llvm-cov</p>
    </div>
</header>
<nav>
    <div class="container">
        <a href='{docs_path}'>Overview</a>
        <a href='{coverage_page_path}'>Coverage Report</a>
        <button id="theme-toggle" class="theme-toggle" aria-label="Toggle theme">
            <span class="theme-icon">🌙</span>
        </button>
    </div>
</nav>
<span class='control'><a href='javascript:next_line()'>next uncovered line (L)</a>, <a href='javascript:next_region()'>next uncovered region (R)</a>, <a href='javascript:next_branch()'>next uncovered branch (B)</a></span>"""
            
            content = re.sub(r'<body>', '<body>' + header_html, content, count=1)
            content = re.sub(r'<h2>Coverage Report</h2><h4>Created: [^<]+</h4>', '', content, count=1)
        else:
            content = re.sub(r"<a href='[^']*'>Overview</a>", f"<a href='{docs_path}'>Overview</a>", content)
            content = re.sub(r"<a href='[^']*'>Coverage Report</a>", f"<a href='{coverage_page_path}'>Coverage Report</a>", content)
        
        content = source_title_pattern.sub(source_title_replacement, content)
        
        def safe_replace(match, orig_replacement):
            quoted = match.group(0)
            if quoted.startswith(("href='http://", "href='https://", "href=\"http://", "href=\"https://", "src='http://", "src='https://", "src=\"http://", "src=\"https://")):
                return quoted
            return orig_replacement
        
        content = re.sub(pattern, lambda m: safe_replace(m, replacement), content)
        content = re.sub(abs_pattern, lambda m: safe_replace(m, replacement), content)
        content = re.sub(r"href='([^']*?)\.(rs|toml|md|txt|json|yml|yaml)\.html'", r"href='\1.html'", content)
        
        html_file.write_text(content, encoding='utf-8')
        files_processed += 1
    
    return files_processed


def restructure_coverage_html(output_dir, package_name, project_root):
    """Restructure llvm-cov HTML output to use workspace-relative paths."""
    import re
    
    html_dir = output_dir / 'html'
    
    if not html_dir.exists():
        print("Warning: html directory not found, skipping restructure")
        return
    
    js_src = html_dir / 'control.js'
    if js_src.exists():
        js_dest = output_dir / 'control.js'
        shutil.copy2(str(js_src), str(js_dest))
    
    coverage_subdir = html_dir / 'coverage'
    if not coverage_subdir.exists():
        print("Warning: coverage subdirectory not found")
        return
    
    # Find the 'src' directory under coverage_subdir
    src_dir = None
    for item in coverage_subdir.rglob('src'):
        if item.is_dir():
            src_dir = item
            break
    
    if src_dir and src_dir.exists():
        for item in src_dir.iterdir():
            dest = output_dir / item.name
            if dest.exists():
                if dest.is_dir():
                    shutil.rmtree(dest)
                else:
                    dest.unlink()
            shutil.move(str(item), str(dest))
        shutil.rmtree(src_dir)
    
    if html_dir.exists():
        shutil.rmtree(html_dir)
    
    rename_rust_html_files(output_dir)
    
    if output_dir.exists():
        files_processed = fix_coverage_html_paths(output_dir, package_name, project_root)
        print(f"  Processed {files_processed} HTML files")
    
    print("  ✓ Restructured HTML files to use workspace-relative paths")


def rename_rust_html_files(coverage_dir):
    """Rename foo.rs.html to foo.html, stripping .rs from file names."""
    import re
    
    renamed = 0
    for html_file in coverage_dir.rglob('*.rs.html'):
        new_name = html_file.with_name(html_file.stem[:-3] + '.html')
        if new_name.exists():
            new_name.unlink()
        html_file.rename(new_name)
        renamed += 1
    
    if renamed > 0:
        print(f"  Renamed {renamed} files from *.rs.html to *.html")
        for html_file in coverage_dir.rglob('*.html'):
            content = html_file.read_text(encoding='utf-8')
            content = re.sub(r"(\.(?:rs|toml|md|txt|json|yml|yaml))\.html'", ".html'", content)
            content = re.sub(r"(\.(?:rs|toml|md|txt|json|yml|yaml))\.html\"", '.html"', content)
            content = re.sub(r"(\.(?:rs|toml|md|txt|json|yml|yaml))\.html>", '.html>', content)
            html_file.write_text(content, encoding='utf-8')


def count_tests_from_list(args):
    """Run cargo test --list and count tests."""
    try:
        result = subprocess.run(
            args,
            capture_output=True,
            text=True,
            check=True
        )
        return len([line for line in result.stdout.split('\n') if ': test' in line])
    except subprocess.CalledProcessError as e:
        print(f"Warning: Failed to count tests for {args}: {e}", file=sys.stderr)
        return 0


def collect_test_statistics():
    """Collect test count breakdown."""
    print("Collecting test statistics...")
    
    lib_tests = count_tests_from_list(['cargo', 'test', '--lib', '--', '--list'])
    doc_tests = count_tests_from_list(['cargo', 'test', '--doc', '--', '--list'])
    
    integration_tests = 0
    test_files = glob.glob('tests/*.rs')
    for test_file in test_files:
        test_name = Path(test_file).stem
        integration_tests += count_tests_from_list(
            ['cargo', 'test', '--test', test_name, '--', '--list']
        )
    
    test_count = lib_tests + doc_tests + integration_tests
    
    fuzz_dir = Path('fuzz/fuzz_targets')
    fuzz_targets = 0
    if fuzz_dir.exists():
        fuzz_targets = len(list(fuzz_dir.glob('*.rs')))
    
    return {
        'test_count': test_count,
        'lib_tests': lib_tests,
        'doc_tests': doc_tests,
        'integration_tests': integration_tests,
        'fuzz_targets': fuzz_targets
    }


def extract_coverage_metrics(coverage_file):
    """Extract coverage metrics from coverage.json."""
    try:
        with open(coverage_file, 'r') as f:
            coverage = json.load(f)
        
        totals = coverage['data'][0]['totals']
        
        return {
            'line_coverage': round(totals['lines']['percent'], 2),
            'function_coverage': round(totals['functions']['percent'], 2),
            'region_coverage': round(totals['regions']['percent'], 2)
        }
    except (IOError, KeyError, IndexError) as e:
        print(f"Error reading coverage data: {e}", file=sys.stderr)
        return {
            'line_coverage': 0.0,
            'function_coverage': 0.0,
            'region_coverage': 0.0
        }


def create_coverage_snapshot(commit_sha, commit_msg, timestamp, test_stats, coverage_metrics, coverage_file):
    """Create a coverage snapshot from current data."""
    try:
        with open(coverage_file, 'r') as f:
            coverage_data = json.load(f)
        
        totals = coverage_data['data'][0]['totals']
        
        return {
            "timestamp": timestamp,
            "commit_sha": commit_sha,
            "commit_message": commit_msg,
            "coverage": {
                "line_coverage": coverage_metrics['line_coverage'],
                "function_coverage": coverage_metrics['function_coverage'],
                "region_coverage": coverage_metrics['region_coverage']
            },
            "tests": {
                "total": test_stats['test_count'],
                "lib_tests": test_stats['lib_tests'],
                "doc_tests": test_stats['doc_tests'],
                "integration_tests": test_stats['integration_tests'],
                "fuzz_targets": test_stats['fuzz_targets']
            },
            "details": {
                "lines": {
                    "covered": totals['lines']['covered'],
                    "total": totals['lines']['count'],
                    "percent": round(totals['lines']['percent'], 2)
                },
                "functions": {
                    "covered": totals['functions']['covered'],
                    "total": totals['functions']['count'],
                    "percent": round(totals['functions']['percent'], 2)
                },
                "regions": {
                    "covered": totals['regions']['covered'],
                    "total": totals['regions']['count'],
                    "percent": round(totals['regions']['percent'], 2)
                }
            }
        }
    except (IOError, KeyError, IndexError) as e:
        print(f"Error creating snapshot: {e}", file=sys.stderr)
        sys.exit(1)


def update_coverage_summary(history):
    """Calculate summary statistics and trends for all metrics."""
    if not history['snapshots']:
        return {}
    
    summary = {}
    
    for metric in ['line_coverage', 'function_coverage', 'region_coverage']:
        values = [s['coverage'][metric] for s in history['snapshots']]
        
        if len(values) >= 2:
            recent = values[-1]
            previous = values[-2]
            change = recent - previous
            
            if abs(change) < 0.1:
                trend = "stable"
            elif change > 0:
                trend = "improving"
            else:
                trend = "degrading"
        else:
            trend = "no_data"
            change = 0
        
        summary[metric] = {
            "latest": values[-1],
            "trend": trend,
            "change_percent": round(abs(change), 2),
            "best": round(max(values), 2),
            "worst": round(min(values), 2),
            "average": round(sum(values) / len(values), 2)
        }
    
    test_counts = [s['tests']['total'] for s in history['snapshots']]
    if len(test_counts) >= 2:
        change = test_counts[-1] - test_counts[-2]
        growth_rate = (change / test_counts[-2]) * 100 if test_counts[-2] > 0 else 0
        
        trend = "growing" if change > 5 else "stable" if change >= -5 else "shrinking"
        
        summary['test_count'] = {
            "latest": test_counts[-1],
            "trend": trend,
            "change_count": change,
            "growth_rate": round(growth_rate, 2)
        }
    
    return summary


def write_stats(test_stats, coverage_metrics, stats_file):
    """Write combined statistics to stats.json."""
    stats = {**test_stats, **coverage_metrics}
    
    stats_file.parent.mkdir(parents=True, exist_ok=True)
    
    with open(stats_file, 'w') as f:
        json.dump(stats, f, indent=2)
    
    print(f"\nGenerated coverage statistics:")
    print(f"  Total tests: {stats['test_count']:,}")
    print(f"    - Library tests: {stats['lib_tests']:,}")
    print(f"    - Doc tests: {stats['doc_tests']:,}")
    print(f"    - Integration tests: {stats['integration_tests']:,}")
    print(f"    - Fuzz targets: {stats['fuzz_targets']}")
    print(f"  Line coverage: {stats['line_coverage']}%")
    print(f"  Function coverage: {stats['function_coverage']}%")
    print(f"  Region coverage: {stats['region_coverage']}%")
    
    print(f"\n✓ Coverage data saved to {stats_file.parent / 'coverage.json'}")
    print(f"✓ Test statistics saved to {stats_file}")


def main():
    """Main execution function."""
    try:
        args = parse_args()
        
        project_root = Path.cwd()
        package_info = get_project_info(args.project, args)
        
        print(f"Generating coverage for project: {args.project}")
        print(f"Output directory: {package_info['docs_dir']}")
        
        # Generate HTML coverage
        if not args.no_html:
            html_dir = generate_html_coverage(package_info, project_root)
        else:
            print("Skipping HTML coverage generation")
            run_coverage(package_info)
            html_dir = package_info["coverage_dir"]
        
        # Collect statistics
        test_stats = collect_test_statistics()
        coverage_metrics = extract_coverage_metrics(package_info["coverage_file"])
        write_stats(test_stats, coverage_metrics, package_info["stats_file"])
        
        # Historical tracking
        if not args.no_history:
            print("\n📊 Updating coverage history...")
            commit_sha, commit_msg, timestamp = get_git_info()
            
            history = load_history(package_info["history_file"])
            snapshot = create_coverage_snapshot(
                commit_sha, commit_msg, timestamp,
                test_stats, coverage_metrics,
                package_info["coverage_file"]
            )
            
            history['snapshots'].append(snapshot)
            history['last_updated'] = timestamp
            
            before_count = len(history['snapshots'])
            history['snapshots'], retention_summary = apply_retention_policy(
                history['snapshots']
            )
            
            history['coverage_summary'] = update_coverage_summary(history)
            
            package_info["history_file"].parent.mkdir(parents=True, exist_ok=True)
            with open(package_info["history_file"], 'w') as f:
                json.dump(history, f, indent=2)
            
            print(f"   Snapshots before retention: {before_count}")
            print(f"   Snapshots retained: {retention_summary['kept']}")
            print(f"   Snapshots removed: {retention_summary['removed']}")
            print(f"\n✅ Coverage history saved to {package_info['history_file']}")
            print(f"   {len(history['snapshots'])} total snapshots")
            print(f"   Commit: {commit_sha[:8]} - {commit_msg}")
            
            summary = history['coverage_summary']
            if summary:
                print(f"\n📈 Coverage Trends:")
                print(f"   Line: {summary['line_coverage']['latest']}% "
                      f"({summary['line_coverage']['trend']})")
                print(f"   Function: {summary['function_coverage']['latest']}% "
                      f"({summary['function_coverage']['trend']})")
                print(f"   Region: {summary['region_coverage']['latest']}% "
                      f"({summary['region_coverage']['trend']})")
                if 'test_count' in summary:
                    print(f"   Tests: {summary['test_count']['latest']:,} "
                          f"({summary['test_count']['trend']}, "
                          f"{summary['test_count']['change_count']:+d})")
        
        print(f"\n✓ HTML coverage reports available at {html_dir}")
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    main()
