#!/usr/bin/env python3
"""
Cross-platform WASM build script for the VB6Interpret playground.

Requirements:
- Python 3.7+
- wasm-pack (installed via: cargo install wasm-pack)
- wasm-opt (optional, installed separately)
"""

import argparse
import os
import platform
import shutil
import subprocess
import sys
from pathlib import Path


def find_executable(name: str) -> str | None:
    if platform.system() == "Windows":
        executable = shutil.which(f"{name}.exe")
        if executable:
            return executable
    return shutil.which(name)


def check_requirements() -> tuple[str, str | None]:
    wasm_pack = find_executable("wasm-pack")
    if not wasm_pack:
        print("Error: wasm-pack not found in PATH", file=sys.stderr)
        print("Install with: cargo install wasm-pack", file=sys.stderr)
        sys.exit(1)

    wasm_opt = find_executable("wasm-opt")
    if not wasm_opt:
        print("Warning: wasm-opt not found; optimization will be skipped")

    return wasm_pack, wasm_opt


def run_command(cmd: list[str], description: str) -> None:
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


def build_wasm(wasm_pack: str, output_dir: Path, no_typescript: bool) -> None:
    cmd = [
        wasm_pack,
        "build",
        "--target",
        "web",
        "--out-dir",
        str(output_dir),
        "--release",
        "--no-default-features",
        "--no-opt",
    ]

    if no_typescript:
        cmd.append("--no-typescript")

    run_command(cmd, "VB6Interpret WASM module")


def optimize_wasm(wasm_opt: str | None, wasm_file: Path) -> None:
    if not wasm_opt or not wasm_file.exists():
        return

    backup_file = wasm_file.with_suffix(".wasm.bak")
    shutil.copy2(wasm_file, backup_file)

    try:
        cmd = [
            wasm_opt,
            "-Oz",
            "--enable-bulk-memory",
            "-o",
            str(wasm_file),
            str(backup_file),
        ]
        run_command(cmd, "WASM optimization")
        backup_file.unlink()
    except Exception:
        shutil.move(backup_file, wasm_file)
        raise


def main() -> None:
    parser = argparse.ArgumentParser(description="Build VB6Interpret WASM module for playground")
    parser.add_argument("--optimize", action="store_true", help="Optimize output with wasm-opt")
    parser.add_argument("--no-typescript", action="store_true", help="Skip TypeScript definitions")
    args = parser.parse_args()

    script_dir = Path(__file__).parent.resolve()
    project_root = script_dir.parent
    workspace_root = project_root.parent.parent
    output_dir = workspace_root / "docs" / "vb6interpret" / "assets" / "wasm"

    os.chdir(project_root)
    wasm_pack, wasm_opt = check_requirements()
    output_dir.mkdir(parents=True, exist_ok=True)

    build_wasm(wasm_pack, output_dir, args.no_typescript)

    if args.optimize:
        optimize_wasm(wasm_opt, output_dir / "vb6interpret_bg.wasm")

    gitignore_file = output_dir / ".gitignore"
    if gitignore_file.exists():
        gitignore_file.unlink()

    print(f"Output written to: {output_dir}")


if __name__ == "__main__":
    main()