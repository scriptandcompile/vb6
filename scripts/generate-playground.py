#!/usr/bin/env python3
"""
Consolidated WASM build script for all VB6 playground projects.
Builds WASM modules using wasm-pack for vb6interpret, vb6parse, and vb6semantic.

Usage:
    python scripts/generate-playground.py                          # Build all
    python scripts/generate-playground.py --project vb6parse       # Build one
    python scripts/generate-playground.py --no-typescript          # Skip all TypeScript
    python scripts/generate-playground.py --optimize               # Optimize with wasm-opt
    python scripts/generate-playground.py --project vb6parse --optimize  # Optimize one
"""

import argparse
import os
import platform
import shutil
import subprocess
import sys
from pathlib import Path

# Project configurations
PROJECTS = {
    "vb6interpret": {
        "name": "vb6interpret",
        "output_dir_name": "vb6interpret",
        "wasm_file": "vb6interpret_bg.wasm",
        "cargo_features": "wasm",
        "no_default_features": True,
    },
    "vb6parse": {
        "name": "vb6parse",
        "output_dir_name": "vb6parse",
        "wasm_file": "vb6parse_bg.wasm",
        "cargo_features": None,
        "no_default_features": False,
    },
    "vb6semantic": {
        "name": "vb6semantic",
        "output_dir_name": "vb6semantic",
        "wasm_file": "vb6semantic_bg.wasm",
        "cargo_features": None,
        "no_default_features": False,
    },
}


def parse_args():
    """Parse command line arguments."""
    parser = argparse.ArgumentParser(
        description="Build WASM modules for VB6 playground projects."
    )
    parser.add_argument(
        "--project", "-p",
        choices=list(PROJECTS.keys()),
        default=None,
        help="Project to build (default: all projects)",
    )
    parser.add_argument(
        "--optimize",
        action="store_true",
        help="Optimize output with wasm-opt (requires wasm-opt to be installed)",
    )
    parser.add_argument(
        "--no-typescript",
        action="store_true",
        help="Skip TypeScript definition generation for all projects",
    )
    return parser.parse_args()


def find_executable(name):
    """Find an executable in PATH, handling Windows .exe extension."""
    if platform.system() == "Windows":
        exe = shutil.which(f"{name}.exe")
        if exe:
            return exe
    return shutil.which(name)


def check_requirements():
    """Check if required tools are installed."""
    wasm_pack = find_executable("wasm-pack")
    if not wasm_pack:
        print("Error: wasm-pack not found in PATH", file=sys.stderr)
        print("Install with: cargo install wasm-pack", file=sys.stderr)
        sys.exit(1)

    wasm_opt = find_executable("wasm-opt")
    if not wasm_opt:
        print("Warning: wasm-opt not found; optimization will be skipped")

    return wasm_pack, wasm_opt


def run_command(cmd, description):
    """Run a command and handle errors."""
    print(f"Building: {description}")
    try:
        result = subprocess.run(cmd, check=True, capture_output=True, text=True)
        if result.stdout:
            print(result.stdout)
    except subprocess.CalledProcessError as error:
        print(f"Error: {description} failed", file=sys.stderr)
        if error.stdout:
            print(error.stdout, file=sys.stderr)
        if error.stderr:
            print(error.stderr, file=sys.stderr)
        sys.exit(1)


def build_wasm(wasm_pack, project_config, output_dir, no_typescript):
    """Build the WASM module for a single project using wasm-pack."""
    cmd = [
        wasm_pack,
        "build",
        "--target", "web",
        "--out-dir", str(output_dir),
        "--release",
        "--no-opt",
    ]

    if project_config["no_default_features"]:
        cmd.append("--no-default-features")

    if no_typescript:
        cmd.append("--no-typescript")

    project = project_config["name"]
    run_command(cmd, f"{project} WASM module")


def optimize_wasm(wasm_opt, wasm_file):
    """Optimize a WASM binary using wasm-opt if available."""
    if not wasm_opt or not wasm_file.exists():
        return

    backup_file = wasm_file.with_suffix(".wasm.bak")
    shutil.copy2(wasm_file, backup_file)

    try:
        cmd = [
            wasm_opt,
            "-Oz",
            "--enable-bulk-memory",
            "--enable-nontrapping-float-to-int",
            "-o", str(wasm_file),
            str(backup_file),
        ]
        run_command(cmd, "WASM optimization")
        backup_file.unlink()

        original_size = backup_file.stat().st_size
        optimized_size = wasm_file.stat().st_size
        savings = original_size - optimized_size
        percent = (savings / original_size) * 100
        print(f"  Saved: {savings:,} bytes ({percent:.1f}%)")

    except Exception:
        shutil.move(backup_file, wasm_file)
        raise


def build_project(wasm_pack, wasm_opt, project_name, config, args):
    """Build a single project's WASM module."""
    print(f"\n{'=' * 50}")
    print(f"Building: {config['name']}")
    print(f"{'=' * 50}")

    repo_root = Path(__file__).resolve().parent.parent
    project_dir = repo_root / "projects" / project_name
    output_dir = repo_root / "docs" / config["output_dir_name"] / "assets" / "wasm"

    if not project_dir.exists():
        print(f"Error: Project directory not found: {project_dir}", file=sys.stderr)
        sys.exit(1)

    os.chdir(project_dir)
    output_dir.mkdir(parents=True, exist_ok=True)

    build_wasm(wasm_pack, config, output_dir, args.no_typescript)

    if args.optimize:
        wasm_file = output_dir / config["wasm_file"]
        optimize_wasm(wasm_opt, wasm_file)

    gitignore_file = output_dir / ".gitignore"
    if gitignore_file.exists():
        gitignore_file.unlink()

    file_count = sum(1 for _ in output_dir.iterdir())
    print(f"Output written to: {output_dir} ({file_count} files)")


def main():
    args = parse_args()

    wasm_pack, wasm_opt = check_requirements()

    projects_to_build = (
        [args.project] if args.project else list(PROJECTS.keys())
    )

    for project_name in projects_to_build:
        config = PROJECTS[project_name]
        build_project(wasm_pack, wasm_opt, project_name, config, args)

    print(f"\n{'=' * 50}")
    print("All done!")
    print(f"{'=' * 50}")


if __name__ == "__main__":
    main()
