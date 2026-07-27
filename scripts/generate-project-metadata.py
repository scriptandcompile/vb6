#!/usr/bin/env python3
"""Generate the workspace project metadata JSON used by the docs hub."""

from __future__ import annotations

import argparse
import importlib.util
import json
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SOURCE_PATH = ROOT / "scripts" / "workspace_projects.py"
OUTPUT_PATH = ROOT / "docs" / "assets" / "data" / "projects.json"


def load_projects() -> list[dict[str, object]]:
    spec = importlib.util.spec_from_file_location("workspace_projects", SOURCE_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"Could not load project metadata source: {SOURCE_PATH}")

    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)

    projects = getattr(module, "PROJECTS", None)
    if not isinstance(projects, list):
        raise RuntimeError("workspace_projects.py must define PROJECTS as a list")

    return projects


def render(projects: list[dict[str, object]]) -> str:
    return json.dumps(projects, indent=2, ensure_ascii=False) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="fail if the generated file is out of date")
    args = parser.parse_args()

    projects = load_projects()
    rendered = render(projects)

    if args.check:
        if not OUTPUT_PATH.exists():
            print(f"missing generated file: {OUTPUT_PATH}", file=sys.stderr)
            return 1

        current = OUTPUT_PATH.read_text(encoding="utf-8")
        if current != rendered:
            print(f"{OUTPUT_PATH} is out of date", file=sys.stderr)
            return 1

        return 0

    OUTPUT_PATH.write_text(rendered, encoding="utf-8")
    print(f"wrote {OUTPUT_PATH}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())