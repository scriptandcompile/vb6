#!/usr/bin/env python3
import argparse
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


def build_msi(repo_root: Path, output_msi: Path) -> None:
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
    subprocess.run([candle, "-arch", "x64", "-out", str(obj_path), str(wxs_path)], check=True)
    subprocess.run(
        [
            light,
            "-ext",
            "WixUIExtension",
            "-ext",
            "WixUtilExtension",
            "-out",
            str(output_msi),
            str(obj_path),
        ],
        check=True,
    )

    print(f"Built MSI at {output_msi}")


def main() -> int:
    parser = argparse.ArgumentParser(description="Build an Aspen MSI installer")
    parser.add_argument("--output", required=True, help="Path to the output MSI file")
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parents[1]
    output_msi = Path(args.output).resolve()
    build_msi(repo_root, output_msi)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
