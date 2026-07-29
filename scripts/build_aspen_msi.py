#!/usr/bin/env python3
import argparse
import re
import shutil
import subprocess
import sys
import tomllib
from pathlib import Path


def load_version(repo_root: Path) -> str:
    cargo_toml = repo_root / "projects" / "aspen" / "Cargo.toml"
    with cargo_toml.open("rb") as handle:
        data = tomllib.load(handle)
    return data["package"]["version"]


def run_wix_command(command: list[str]) -> None:
    result = subprocess.run(command, text=True, capture_output=True)
    if result.stdout:
        print(result.stdout, end="")
    if result.stderr:
        print(result.stderr, end="", file=sys.stderr)
    if result.returncode != 0:
        raise subprocess.CalledProcessError(
            result.returncode,
            result.args,
            output=result.stdout,
            stderr=result.stderr,
        )


def print_wxs_context(wxs_path: Path, stderr: str) -> None:
    match = re.search(r"\.wxs\((\d+)\)", stderr)
    if not match:
        return

    line_num = int(match.group(1))
    lines = wxs_path.read_text(encoding="utf-8").splitlines()
    start = max(1, line_num - 3)
    end = min(len(lines), line_num + 3)

    print(f"\nWiX source context around line {line_num} ({wxs_path}):", file=sys.stderr)
    for i in range(start, end + 1):
        marker = ">" if i == line_num else " "
        print(f"{marker} {i:4}: {lines[i - 1]}", file=sys.stderr)


def print_registry_row_context(obj_path: Path, registry_key: str) -> None:
    if not obj_path.exists():
        return

    text = obj_path.read_text(encoding="utf-8", errors="replace")
    lines = text.splitlines()
    hit = None
    for idx, line in enumerate(lines):
        if registry_key in line:
            hit = idx
            break

    if hit is None:
        print(f"\nCould not find {registry_key} in {obj_path}", file=sys.stderr)
        return

    start = max(0, hit - 6)
    end = min(len(lines), hit + 7)
    print(f"\nRegistry row context for {registry_key} in {obj_path}:", file=sys.stderr)
    for i in range(start, end):
        marker = ">" if i == hit else " "
        print(f"{marker} {i + 1:4}: {lines[i]}", file=sys.stderr)


def emit_ice_diagnostics(stderr: str, wxs_path: Path, obj_path: Path) -> None:
    key_match = re.search(r"Key\(s\):\s*([A-Za-z0-9_]+)", stderr)
    if key_match:
        print_registry_row_context(obj_path, key_match.group(1))
    print_wxs_context(wxs_path, stderr)


def build_msi(repo_root: Path, output_msi: Path, diagnostics: bool = False) -> None:
    version = load_version(repo_root)
    version_parts = version.split(".")
    if len(version_parts) < 3:
        raise ValueError(f"Invalid version '{version}'")
    wix_version = ".".join(version_parts[:3] + ["0"])

    package_dir = repo_root / "dist" / "aspen-windows"
    if not package_dir.exists():
        raise FileNotFoundError("Expected packaged Aspen directory at dist/aspen-windows")

    binary_path = package_dir / "bin" / "aspen.exe"
    cmd_path = package_dir / "bin" / "aspen.cmd"
    ps1_path = package_dir / "bin" / "aspen.ps1"

    if not binary_path.exists():
        raise FileNotFoundError("Expected aspen.exe in dist/aspen-windows/bin")
    if not cmd_path.exists():
        raise FileNotFoundError("Expected aspen.cmd in dist/aspen-windows/bin")
    if not ps1_path.exists():
        raise FileNotFoundError("Expected aspen.ps1 in dist/aspen-windows/bin")

    output_msi.parent.mkdir(parents=True, exist_ok=True)
    obj_dir = output_msi.parent / "wixobj"
    obj_dir.mkdir(parents=True, exist_ok=True)

    template_path = repo_root / "projects" / "aspen" / "installer" / "aspen-installer.wxs"
    if not template_path.exists():
        raise FileNotFoundError(f"Expected WiX template at {template_path}")

    wxs_text = template_path.read_text(encoding="utf-8")
    wxs_text = wxs_text.replace("__VERSION__", wix_version)
    wxs_text = wxs_text.replace("__BINARY_PATH__", str(binary_path))
    wxs_text = wxs_text.replace("__CMD_PATH__", str(cmd_path))
    wxs_text = wxs_text.replace("__PS1_PATH__", str(ps1_path))

    wxs_path = output_msi.parent / "aspen-installer.wxs"
    wxs_path.write_text(wxs_text, encoding="utf-8")

    candle = shutil.which("candle")
    light = shutil.which("light")
    if not candle or not light:
        raise FileNotFoundError("WiX Toolset is not installed or not on PATH")

    obj_path = obj_dir / "aspen-installer.wixobj"
    candle_cmd = [candle, "-arch", "x64", "-out", str(obj_path), str(wxs_path)]
    if diagnostics:
        candle_cmd.insert(1, "-v")
    run_wix_command(candle_cmd)

    light_cmd = [
        light,
        "-ext",
        "WixUIExtension",
        "-ext",
        "WixUtilExtension",
        "-out",
        str(output_msi),
        str(obj_path),
    ]
    if diagnostics:
        light_cmd[1:1] = ["-v", "-pdbout", str(obj_dir / "aspen-installer.wixpdb")]

    try:
        run_wix_command(light_cmd)
    except subprocess.CalledProcessError as exc:
        if diagnostics and isinstance(exc.stderr, str):
            emit_ice_diagnostics(exc.stderr, wxs_path, obj_path)
        raise

    print(f"Built MSI at {output_msi}")


def main() -> int:
    parser = argparse.ArgumentParser(description="Build an Aspen MSI installer")
    parser.add_argument("--output", required=True, help="Path to the output MSI file")
    parser.add_argument(
        "--diagnostics",
        action="store_true",
        help="Enable verbose WiX output and print ICE diagnostics on failure",
    )
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parents[1]
    output_msi = Path(args.output).resolve()
    build_msi(repo_root, output_msi, diagnostics=args.diagnostics)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
