#!/usr/bin/env python3
"""
Generate benchmark data for any Rust project with Criterion benchmarks.
Project-agnostic script that accepts a project name and manages benchmark history.

Usage:
    python generate-benchmarks.py --project vb6parse
    python generate-benchmarks.py --project <project_name> [--force]
"""

import argparse
import json
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

# Retention policy
RETENTION_DAYS_FULL = 30
RETENTION_DAYS_WEEKLY = 180
RETENTION_DAYS_MONTHLY = 365

CI_COMMIT_PATTERNS = [
    r"\bci\b",
    r"\[skip\s+ci\]",
    r"github\s*actions",
    r"workflow",
    r"merge",
    r"update\s+benchmark\s+data",
]


def parse_args():
    parser = argparse.ArgumentParser(
        description="Generate benchmark data for a Rust project with Criterion benchmarks."
    )
    parser.add_argument(
        "--project",
        required=True,
        help="Project name (e.g., vb6parse). Must match directory under projects/ and docs/assets/data/.",
    )
    parser.add_argument(
        "--force",
        action="store_true",
        help="Force re-running benchmarks even if recent data exists.",
    )
    return parser.parse_args()


def get_project_dir(project_name):
    """Find the project directory under projects/<project_name>/."""
    repo_root = Path(__file__).resolve().parent.parent
    project_dir = repo_root / "projects" / project_name
    if not project_dir.exists():
        print(f"Error: Project directory not found: {project_dir}", file=sys.stderr)
        sys.exit(1)
    benches_dir = project_dir / "benches"
    if not benches_dir.exists():
        print(
            f"Error: No benches/ directory found in {project_dir}", file=sys.stderr
        )
        sys.exit(1)
    return project_dir


def get_output_paths(project_name, repo_root=None):
    """Get the output paths for benchmark data files."""
    if repo_root is None:
        repo_root = Path(__file__).resolve().parent.parent
    data_dir = repo_root / "docs" / "assets" / "data" / project_name
    return {
        "snapshot": data_dir / "benchmarks.json",
        "history": data_dir / "benchmarks-history.json",
        "data_dir": data_dir,
    }


def run_cmd(cmd, cwd=None, check=True):
    return subprocess.run(
        cmd, cwd=cwd, capture_output=True, text=True, check=check
    )


def run_benchmarks(project_dir):
    """Run cargo benchmarks for the specified project."""
    print(f"Running benchmarks for project: {project_dir.name}")
    print("Directory:", project_dir)

    criterion_dir = project_dir / "target" / "criterion"
    if criterion_dir.exists():
        print("Clearing existing Criterion data...")
        import shutil
        shutil.rmtree(criterion_dir, ignore_errors=True)

    try:
        result = subprocess.run(
            ["cargo", "bench", "--message-format=json"],
            cwd=project_dir,
            capture_output=True,
            text=True,
        )
        if result.returncode != 0:
            print(f"Benchmark exited with code {result.returncode}", file=sys.stderr)
            if result.stderr:
                print(result.stderr[:500], file=sys.stderr)
            sys.exit(1)
        print("Benchmarks completed successfully")
    except FileNotFoundError:
        print("Error: cargo not found. Make sure Rust is installed.", file=sys.stderr)
        sys.exit(1)


def aggregate_benchmark_data(project_dir):
    """Aggregate benchmark data from Criterion output."""
    print("Aggregating benchmark data from Criterion output...")
    criterion_dir = project_dir / "target" / "criterion"
    benchmarks = []

    if not criterion_dir.exists():
        print("Warning: Criterion directory not found", file=sys.stderr)
        return benchmarks

    for estimates_file in criterion_dir.rglob("**/new/estimates.json"):
        benchmark_name = estimates_file.parent.parent.name

        try:
            with open(estimates_file, "r") as f:
                data = json.load(f)

            mean = data.get("mean", {})
            median = data.get("median", {})
            std_dev = data.get("std_dev", {})

            benchmarks.append(
                {
                    "name": benchmark_name,
                    "mean": mean.get("point_estimate", 0),
                    "median": median.get("point_estimate", 0),
                    "std_dev": std_dev.get("point_estimate", 0),
                    "unit": "ns",
                }
            )
        except (json.JSONDecodeError, IOError) as e:
            print(f"Warning: Failed to read {estimates_file}: {e}", file=sys.stderr)

    benchmarks.sort(key=lambda x: x["name"])
    print(f"Aggregated {len(benchmarks)} benchmarks")
    return benchmarks


def is_ci_commit_message(message):
    """Return True if a commit message appears to be CI-related."""
    import re
    ci_regex = re.compile("|".join(CI_COMMIT_PATTERNS), re.IGNORECASE)
    return bool(ci_regex.search((message or "").strip()))


def get_git_info():
    """Get nearest non-CI git commit information."""
    try:
        log_output = run_cmd(
            ["git", "log", "--pretty=format:%H%x1f%s", "-n", "500"],
            capture_output=True,
            text=True,
            check=True,
        ).stdout.splitlines()

        selected_sha = None
        selected_msg = None

        for line in log_output:
            parts = line.split("\x1f", 1)
            if len(parts) != 2:
                continue

            commit_sha, commit_msg = parts[0].strip(), parts[1].strip()
            if not commit_sha:
                continue

            if not is_ci_commit_message(commit_msg):
                selected_sha = commit_sha
                selected_msg = commit_msg or "No commit message"
                break

        if not selected_sha:
            selected_sha = run_cmd(
                ["git", "rev-parse", "HEAD"],
                capture_output=True,
                text=True,
                check=True,
            ).stdout.strip()
            selected_msg = run_cmd(
                ["git", "log", "-1", "--pretty=%s"],
                capture_output=True,
                text=True,
                check=True,
            ).stdout.strip() or "No commit message"

        if selected_sha and selected_msg:
            print(f"Using commit {selected_sha[:8]} - {selected_msg}")

        timestamp = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
        return selected_sha, selected_msg, timestamp
    except subprocess.CalledProcessError:
        return "unknown", "No git information available", datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def load_history(history_file):
    """Load existing benchmark history or create new."""
    if history_file.exists():
        try:
            with open(history_file, "r") as f:
                return json.load(f)
        except (json.JSONDecodeError, IOError) as e:
            print(f"Warning: Failed to load history, starting fresh: {e}", file=sys.stderr)

    return {
        "version": "1.0",
        "last_updated": "",
        "snapshots": [],
        "benchmarks_summary": {},
    }


def apply_retention_policy(snapshots):
    """Apply retention policy to limit history size."""
    if not snapshots:
        return [], {"removed": 0, "kept": 0}

    now = datetime.now(timezone.utc)
    original_count = len(snapshots)

    parsed_snapshots = []
    for snapshot in snapshots:
        try:
            timestamp_str = snapshot["timestamp"].replace("Z", "+00:00")
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

    retention_stats = {"full": 0, "weekly": 0, "monthly": 0, "quarterly": 0}

    for timestamp, age_days, snapshot in parsed_snapshots:
        if age_days <= RETENTION_DAYS_FULL:
            retained.append(snapshot)
            retention_stats["full"] += 1
        elif age_days <= RETENTION_DAYS_WEEKLY:
            week_key = (timestamp.year, timestamp.isocalendar()[1])
            if week_key not in seen_weeks:
                retained.append(snapshot)
                seen_weeks.add(week_key)
                retention_stats["weekly"] += 1
        elif age_days <= RETENTION_DAYS_MONTHLY:
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

    retained_sorted = sorted(retained, key=lambda s: s["timestamp"])

    summary = {
        "removed": original_count - len(retained_sorted),
        "kept": len(retained_sorted),
        "breakdown": retention_stats,
    }

    return retained_sorted, summary


def calculate_trend(history_data):
    """Calculate trend for a specific benchmark."""
    if not history_data or len(history_data) < 2:
        return None

    latest = history_data[-1]
    previous = history_data[-2]

    if previous == 0:
        return None

    change_percent = ((latest - previous) / previous) * 100

    if abs(change_percent) < 0.5:
        direction = "stable"
    elif change_percent < 0:
        direction = "improving"
    else:
        direction = "degrading"

    return {"direction": direction, "change_percent": round(change_percent, 2)}


def update_benchmarks_summary(history, benchmarks):
    """Update the benchmarks summary with historical data and trends."""
    summary = {}

    for benchmark in benchmarks:
        name = benchmark["name"]

        history_data = []
        for snapshot in history["snapshots"]:
            for bench in snapshot["benchmarks"]:
                if bench["name"] == name:
                    history_data.append(
                        {
                            "timestamp": snapshot["timestamp"],
                            "mean": bench["mean"],
                            "median": bench["median"],
                            "std_dev": bench["std_dev"],
                        }
                    )
                    break

        mean_values = [h["mean"] for h in history_data]
        trend = calculate_trend(mean_values) if len(mean_values) >= 2 else None

        summary[name] = {
            "history": history_data[-10:],
            "trend": trend,
            "latest": {
                "mean": benchmark["mean"],
                "median": benchmark["median"],
                "std_dev": benchmark["std_dev"],
            },
        }

    return summary


def write_benchmark_data(benchmarks, output_paths):
    """Write current benchmark data and update history."""
    # Write current snapshot
    output = {"benchmarks": benchmarks, "count": len(benchmarks)}
    output_paths["snapshot"].parent.mkdir(parents=True, exist_ok=True)

    with open(output_paths["snapshot"], "w") as f:
        json.dump(output, f, indent=2)

    print(f"\nGenerated benchmark data for {len(benchmarks)} benchmarks")
    print(f"  Written to {output_paths['snapshot']}")

    commit_sha, commit_msg, timestamp = get_git_info()

    history = load_history(output_paths["history"])

    snapshot = {
        "timestamp": timestamp,
        "commit_sha": commit_sha,
        "commit_message": commit_msg,
        "benchmarks": benchmarks,
    }

    history["snapshots"].append(snapshot)
    history["last_updated"] = timestamp

    print("\nApplying retention policy...")
    before_count = len(history["snapshots"])
    history["snapshots"], retention_summary = apply_retention_policy(
        history["snapshots"]
    )

    print(f"  Snapshots before retention: {before_count}")
    print(f"  Snapshots retained: {retention_summary['kept']}")
    print(f"  Snapshots removed: {retention_summary['removed']}")

    history["benchmarks_summary"] = update_benchmarks_summary(history, benchmarks)

    with open(output_paths["history"], "w") as f:
        json.dump(history, f, indent=2)

    snapshot_count = len(history["snapshots"])
    print(f"\nUpdated benchmark history ({snapshot_count} snapshots)")
    print(f"  Written to {output_paths['history']}")
    print(f"  Commit: {commit_sha[:8]} - {commit_msg}")

    trend_counts = {"improving": 0, "stable": 0, "degrading": 0, "no_data": 0}
    for entry in history["benchmarks_summary"].values():
        if entry["trend"]:
            trend_counts[entry["trend"]["direction"]] += 1
        else:
            trend_counts["no_data"] += 1

    if trend_counts["improving"] + trend_counts["degrading"] + trend_counts["stable"] > 0:
        print("\nPerformance Trends:")
        print(f"  Improving: {trend_counts['improving']}")
        print(f"  Stable: {trend_counts['stable']}")
        print(f"  Degrading: {trend_counts['degrading']}")
        if trend_counts["no_data"] > 0:
            print(f"  No historical data: {trend_counts['no_data']}")


def main():
    args = parse_args()
    project_name = args.project

    project_dir = get_project_dir(project_name)
    output_paths = get_output_paths(project_name)

    run_benchmarks(project_dir)
    benchmarks = aggregate_benchmark_data(project_dir)

    if not benchmarks:
        print("Error: No benchmark data collected", file=sys.stderr)
        sys.exit(1)

    write_benchmark_data(benchmarks, output_paths)
    print(f"\nDone! Project '{project_name}' benchmarks generated.")


if __name__ == "__main__":
    main()
