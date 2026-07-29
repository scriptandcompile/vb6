#!/usr/bin/env python3
import argparse
import shutil
import sys
from pathlib import Path


def write_text(path: Path, content: str) -> None:
    path.write_text(content, encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description="Package Aspen for Windows release")
    parser.add_argument("--binary", required=True, help="Path to the built aspen.exe")
    parser.add_argument("--output", required=True, help="Destination directory for the packaged files")
    args = parser.parse_args()

    binary_path = Path(args.binary).resolve()
    output_dir = Path(args.output).resolve()
    bin_dir = output_dir / "bin"

    if not binary_path.exists():
        print(f"error: expected binary at {binary_path}", file=sys.stderr)
        return 1

    bin_dir.mkdir(parents=True, exist_ok=True)
    shutil.copy2(binary_path, bin_dir / "aspen.exe")

    write_text(
        bin_dir / "aspen.cmd",
        "@echo off\r\nsetlocal\r\n\"%~dp0aspen.exe\" %*\r\n",
    )
    write_text(
        bin_dir / "aspen.ps1",
        "$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path\n"
        "& \"$scriptDir/aspen.exe\" @args\n",
    )

    write_text(
        output_dir / "README.txt",
        "Aspen Windows release\n"
        "=====================\n\n"
        "This package contains a portable Aspen installation for Windows.\n"
        "To use it from any terminal, add the 'bin' directory to your PATH\n"
        "and then run 'aspen' from Command Prompt or PowerShell.\n\n"
        "Example:\n"
        "  setx PATH \"%PATH%;C:\\path\\to\\aspen\\bin\"\n"
        "  aspen --help\n",
    )

    print(f"Packaged Aspen into {output_dir}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
