#!/usr/bin/env python3
"""
Generate coverage data and test statistics for VB6Parse with historical tracking.
Cross-platform script for Windows and Linux.
"""

import json
import os
import sys
import subprocess
import glob
from pathlib import Path
from datetime import datetime, timezone

# Configuration
HISTORY_FILE = Path("docs/vb6parse/assets/data/coverage-history.json")
COVERAGE_FILE = Path("docs/vb6parse/assets/data/coverage.json")
STATS_FILE = Path("docs/vb6parse/assets/data/stats.json")
RETENTION_DAYS_FULL = 30
RETENTION_DAYS_WEEKLY = 180
RETENTION_DAYS_MONTHLY = 365


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
        # If git is not available or not a git repo
        return "unknown", "No git information available", datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def load_history():
    """Load existing coverage history or create new."""
    if HISTORY_FILE.exists():
        try:
            with open(HISTORY_FILE, 'r') as f:
                return json.load(f)
        except (json.JSONDecodeError, IOError) as e:
            print(f"Warning: Failed to load history, starting fresh: {e}", file=sys.stderr)
    
    return {
        "version": "1.0",
        "last_updated": "",
        "snapshots": [],
        "coverage_summary": {}
    }


def apply_retention_policy(snapshots):
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
    
    # Parse all valid snapshots with their timestamps
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
    
    # Sort by timestamp (newest first for grouping)
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
        # Keep all from last 30 days
        if age_days <= RETENTION_DAYS_FULL:
            retained.append(snapshot)
            retention_stats["full"] += 1
        # Keep one per week for 31-180 days (ISO week)
        elif age_days <= RETENTION_DAYS_WEEKLY:
            week_key = (timestamp.year, timestamp.isocalendar()[1])
            if week_key not in seen_weeks:
                retained.append(snapshot)
                seen_weeks.add(week_key)
                retention_stats["weekly"] += 1
        # Keep one per month for 181-365 days
        elif age_days <= RETENTION_DAYS_MONTHLY:
            month_key = (timestamp.year, timestamp.month)
            if month_key not in seen_months:
                retained.append(snapshot)
                seen_months.add(month_key)
                retention_stats["monthly"] += 1
        # Keep one per quarter beyond 365 days
        else:
            quarter = (timestamp.month - 1) // 3 + 1
            quarter_key = (timestamp.year, quarter)
            if quarter_key not in seen_quarters:
                retained.append(snapshot)
                seen_quarters.add(quarter_key)
                retention_stats["quarterly"] += 1
    
    # Return in chronological order (oldest first)
    retained_sorted = sorted(retained, key=lambda s: s['timestamp'])
    
    summary = {
        "removed": original_count - len(retained_sorted),
        "kept": len(retained_sorted),
        "breakdown": retention_stats
    }
    
    return retained_sorted, summary


def run_coverage():
    """Run cargo llvm-cov to generate coverage data."""
    print("Generating coverage data...")
    
    COVERAGE_FILE.parent.mkdir(parents=True, exist_ok=True)
    
    try:
        subprocess.run(
            ["cargo", "llvm-cov", "--package", "vb6parse", "--lib", "--tests", "--json", "--output-path", str(COVERAGE_FILE)],
            check=True
        )
    except subprocess.CalledProcessError as e:
        print(f"Warning: cargo llvm-cov failed ({e})", file=sys.stderr)
        if not COVERAGE_FILE.exists():
            print("No existing coverage.json found. Exiting.", file=sys.stderr)
            sys.exit(1)
        print(f"Using existing coverage.json: {COVERAGE_FILE}")


def generate_html_coverage():
    """Generate HTML coverage reports using llvm-cov."""
    print("Generating HTML coverage reports...")
    
    output_dir = Path('docs/vb6parse/assets/coverage')
    output_dir.mkdir(parents=True, exist_ok=True)
    
    try:
        subprocess.run(
            [
                "cargo", 
                "llvm-cov", 
                "--package", "vb6parse",
                "--lib",
                "--tests",
                "--html",
                "--output-dir", 
                str(output_dir)
            ],
            check=True
        )
        
        # Restructure the output to use workspace-relative paths
        restructure_coverage_html(output_dir)
        
        print(f"✓ HTML coverage reports generated in {output_dir}")
        return output_dir
    except subprocess.CalledProcessError as e:
        print(f"Warning: cargo llvm-cov --html failed ({e})", file=sys.stderr)
    
    src_dir = output_dir / 'src'
    if src_dir.exists():
        html_files = list(src_dir.rglob('*.html'))
        if html_files:
            print(f"Reprocessing {len(html_files)} existing HTML files...")
            fix_coverage_html_paths(src_dir, output_dir)
        else:
            print("Warning: No existing HTML files found in src/", file=sys.stderr)
    else:
        print("Warning: No existing src/ directory found", file=sys.stderr)
    
    return output_dir


def fix_coverage_html_paths(src_dir, output_dir):
    """Fix paths in existing coverage HTML files in src/ directory."""
    import re
    from pathlib import Path
    
    # Patterns to match coverage-generated paths
    project_root = Path.cwd()
    pattern = re.compile(r"(href|src)='coverage/[^']*?/src/", re.IGNORECASE)
    replacement = r"\1='src/"
    abs_pattern = re.compile(r"(href|src)='[^']*?/" + re.escape(project_root.name) + r"/src/", re.IGNORECASE)
    source_title_pattern = re.compile(
        r"<div class='source-name-title'><pre>.*?/" + re.escape(project_root.name) + r"/(src/[^<]+)</pre></div>",
        re.IGNORECASE
    )
    github_url = "https://github.com/scriptandcompile/vb6/tree/master/projects/vb6parse/"
    
    def source_title_replacement(match):
        relative_path = match.group(1)
        return f"<div class='source-name-title'><a href='{github_url}{relative_path}'>{relative_path}</a></div>"
    
    files_processed = 0
    for html_file in src_dir.rglob('*.html'):
        content = html_file.read_text(encoding='utf-8')
        
        relative_to_src = html_file.relative_to(src_dir)
        depth_from_src = len(relative_to_src.parts) - 1
        
        # CSS/JS are at docs/assets/ - from coverage/src/: 4 levels up to docs/
        # coverage/src/ -> coverage/ -> assets/ -> vb6parse/ -> docs/
        css_depth = depth_from_src + 4
        css_base = '../' * css_depth + 'assets/css/'
        
        control_up = depth_from_src + 1
        control_path = '../' * control_up + 'control.js'
        
        js_path = '../' * css_depth + 'assets/js/'
        
        css_link = f"<link rel='stylesheet' type='text/css' href='{css_base}llvm-cov.css'>"
        style_link = f"<link rel='stylesheet' type='text/css' href='{css_base}style.css'>"
        theme_script_src = f'<script src="{js_path}theme-switcher.js"></script>'
        
        # Replace ALL CSS links - first one is llvm-cov, second is style
        css_links = re.findall(r"<link rel='stylesheet' type='text/css' href='[^']*\.css'>", content)
        if len(css_links) >= 1:
            content = content.replace(css_links[0], css_link, 1)
        if len(css_links) >= 2:
            content = content.replace(css_links[1], style_link, 1)
        # Add style link if we only had llvm-cov
        if 'style.css' not in content and len(css_links) == 1:
            content = content.replace(css_link, css_link + '\n' + style_link)
        
        # Replace ALL control.js src attributes
        control_matches = re.findall(r"src='[^']*control\.js'", content)
        for match in control_matches:
            content = content.replace(match, f"src='{control_path}'", 1)
        
        # Replace theme-switcher.js script tag (handle both quote styles)
        theme_matches = re.findall(r"<script src=[\"'][^\"']*theme-switcher\.js[\"']></script>", content)
        for match in theme_matches:
            content = content.replace(match, theme_script_src, 1)
        if 'theme-switcher.js' not in content:
            content = content.replace('<body>', '<body>\n' + theme_script_src)
        
        # Remove duplicate control spans that appear after theme-switcher
        # Match from > after </script> to </span> of the control span
        content = re.sub(r"</script>\s*<span class='control'[^>]*>.*?</span>", "</script>", content, flags=re.DOTALL)
        
        # Calculate nav paths - from src/ to vb6parse/ is 3 levels up
        # coverage/src/ → coverage/ → assets/ → vb6parse/
        nav_up_depth = depth_from_src + 3
        docs_path = '../' * nav_up_depth + 'index.html'
        coverage_page_path = '../' * nav_up_depth + 'coverage.html'
        
        # Check if header already has our custom nav structure
        has_custom_header = 'VB6Parse Coverage Report</h1>' in content
        
        if not has_custom_header:
            header_html = f"""<header>
    <div class="container">
        <h1>VB6Parse Coverage Report</h1>
        <p class="tagline">Generated from llvm-cov</p>
    </div>
</header>
<nav>
    <div class="container">
        <a href='{coverage_page_path}'>Coverage Report</a>
        <a href='{docs_path}'>Overview</a>
        <button id="theme-toggle" class="theme-toggle" aria-label="Toggle theme">
            <span class="theme-icon">🌙</span>
        </button>
    </div>
</nav>
<span class='control'><a href='javascript:next_line()'>next uncovered line (L)</a>, <a href='javascript:next_region()'>next uncovered region (R)</a>, <a href='javascript:next_branch()'>next uncovered branch (B)</a></span>"""
            
            content = re.sub(r'<body>', '<body>' + header_html, content, count=1)
            content = re.sub(r'<h2>Coverage Report</h2><h4>Created: [^<]+</h4>', '', content, count=1)
        else:
            # Update existing header nav paths
            content = re.sub(r"<a href='[^']*'>Coverage Report</a>", f"<a href='{coverage_page_path}'>Coverage Report</a>", content)
            content = re.sub(r"<a href='[^']*'>Overview</a>", f"<a href='{docs_path}'>Overview</a>", content)
        
        content = source_title_pattern.sub(source_title_replacement, content)
        content = pattern.sub(replacement, content)
        content = abs_pattern.sub(replacement, content)
        content = re.sub(r"href='([^']*?)\.(rs|toml|md|txt|json|yml|yaml)\.html'", r"href='\1.html'", content)
        
        html_file.write_text(content, encoding='utf-8')
        files_processed += 1
    
    return files_processed


def restructure_coverage_html(output_dir):
    """Restructure llvm-cov HTML output to use workspace-relative paths."""
    import shutil
    import re
    
    html_dir = output_dir / 'html'
    
    if not html_dir.exists():
        print("Warning: html directory not found, skipping restructure")
        return
    
    # Copy control.js to coverage root (provides interactive functionality)
    js_src = html_dir / 'control.js'
    if js_src.exists():
        js_dest = output_dir / 'control.js'
        shutil.copy2(str(js_src), str(js_dest))
    
    # Find the workspace root in the nested structure
    coverage_subdir = html_dir / 'coverage'
    if not coverage_subdir.exists():
        print("Warning: coverage subdirectory not found")
        return
    
    # Get the current working directory to find where the project path starts
    project_root = Path.cwd()
    
    # Navigate through the nested path to find the src directory
    nested_path = coverage_subdir
    for part in project_root.parts:
        nested_path = nested_path / part
        if not nested_path.exists():
            break
    
    # Find where 'src' directory is in the nested structure
    src_path = None
    for root, dirs, files in os.walk(coverage_subdir):
        if root.endswith('/src') or '/src/' in root:
            # Found a src directory, use its parent as the base
            root_path = Path(root)
            # Go up to find the project root (where src, tests, etc. are)
            while root_path.name not in ['coverage']:
                if (root_path / 'src').exists() or root_path.name == str(project_root.name):
                    src_path = root_path
                    break
                root_path = root_path.parent
            if src_path:
                break
    
    if not src_path:
        print("Warning: Could not find src directory in nested structure")
        # Try to find any directory that contains the project name
        for root, dirs, files in os.walk(coverage_subdir):
            if project_root.name in root:
                src_path = Path(root)
                break
    
    if src_path and src_path.exists():
        # Move src directory to output_dir/src
        for item in src_path.iterdir():
            dest = output_dir / item.name
            if dest.exists():
                if dest.is_dir():
                    shutil.rmtree(dest)
                else:
                    dest.unlink()
            shutil.move(str(item), str(dest))
    
    # Clean up the html directory
    if html_dir.exists():
        shutil.rmtree(html_dir)
    
    # Fix paths in all HTML files in src/ directory
    src_dir = output_dir / 'src'
    if src_dir.exists():
        files_processed = fix_coverage_html_paths(src_dir, output_dir)
        print(f"  Processed {files_processed} HTML files")
    
    print("  ✓ Restructured HTML files to use workspace-relative paths")


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
    
    # Get library tests (from src/)
    lib_tests = count_tests_from_list(['cargo', 'test', '--lib', '--', '--list'])
    
    # Get doc tests
    doc_tests = count_tests_from_list(['cargo', 'test', '--doc', '--', '--list'])
    
    # Get integration tests by counting each test file
    integration_tests = 0
    test_files = glob.glob('tests/*.rs')
    for test_file in test_files:
        test_name = Path(test_file).stem  # Remove .rs extension
        integration_tests += count_tests_from_list(
            ['cargo', 'test', '--test', test_name, '--', '--list']
        )
    
    # Total test count
    test_count = lib_tests + doc_tests + integration_tests
    
    # Count fuzz targets
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


def extract_coverage_metrics():
    """Extract coverage metrics from coverage.json."""
    try:
        with open(COVERAGE_FILE, 'r') as f:
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


def create_coverage_snapshot(commit_sha, commit_msg, timestamp, test_stats, coverage_metrics):
    """Create a coverage snapshot from current data."""
    try:
        with open(COVERAGE_FILE, 'r') as f:
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
            
            # Coverage changes are small, use tight threshold
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
    
    # Test count tracking
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


def write_stats(test_stats, coverage_metrics):
    """Write combined statistics to stats.json."""
    stats = {**test_stats, **coverage_metrics}
    
    STATS_FILE.parent.mkdir(parents=True, exist_ok=True)
    
    with open(STATS_FILE, 'w') as f:
        json.dump(stats, f, indent=2)
    
    # Print summary
    print(f"\nGenerated coverage statistics:")
    print(f"  Total tests: {stats['test_count']:,}")
    print(f"    - Library tests: {stats['lib_tests']:,}")
    print(f"    - Doc tests: {stats['doc_tests']:,}")
    print(f"    - Integration tests: {stats['integration_tests']:,}")
    print(f"    - Fuzz targets: {stats['fuzz_targets']}")
    print(f"  Line coverage: {stats['line_coverage']}%")
    print(f"  Function coverage: {stats['function_coverage']}%")
    print(f"  Region coverage: {stats['region_coverage']}%")
    
    print(f"\n✓ Coverage data saved to {COVERAGE_FILE}")
    print(f"✓ Test statistics saved to {STATS_FILE}")


def main():
    """Main execution function."""
    try:
        # Generate both JSON and HTML outputs
        run_coverage()
        html_dir = generate_html_coverage()
        
        # Continue with statistics
        test_stats = collect_test_statistics()
        coverage_metrics = extract_coverage_metrics()
        write_stats(test_stats, coverage_metrics)
        
        # Historical tracking
        print("\n📊 Updating coverage history...")
        commit_sha, commit_msg, timestamp = get_git_info()
        
        # Load and update history
        history = load_history()
        snapshot = create_coverage_snapshot(
            commit_sha, commit_msg, timestamp,
            test_stats, coverage_metrics
        )
        
        history['snapshots'].append(snapshot)
        history['last_updated'] = timestamp
        
        # Apply retention policy
        before_count = len(history['snapshots'])
        history['snapshots'], retention_summary = apply_retention_policy(
            history['snapshots']
        )
        
        # Update summary with trends
        history['coverage_summary'] = update_coverage_summary(history)
        
        # Write history file
        HISTORY_FILE.parent.mkdir(parents=True, exist_ok=True)
        with open(HISTORY_FILE, 'w') as f:
            json.dump(history, f, indent=2)
        
        # Print summary
        print(f"   Snapshots before retention: {before_count}")
        print(f"   Snapshots retained: {retention_summary['kept']}")
        print(f"   Snapshots removed: {retention_summary['removed']}")
        print(f"\n✅ Coverage history saved to {HISTORY_FILE}")
        print(f"   {len(history['snapshots'])} total snapshots")
        print(f"   Commit: {commit_sha[:8]} - {commit_msg}")
        
        # Show trends
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
