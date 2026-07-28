# Aspen

Aspen is a VB6 analysis tool in the spirit of `cargo check`, `cargo fmt`, etc.
It recursively discovers and validates VB6 project (`.vbp`) files, checking for
missing references, parse errors, and non-English source files.

## Commands

### `check`

Validate one or more VB6 projects.

```
aspen check [OPTIONS] [project path]
```

If `[project path]` is a directory, Aspen recursively searches for `.vbp` files
and checks each one. If it points to a single `.vbp` file, only that project is
checked. Defaults to the current directory.

| Flag | Alias | Description |
|------|-------|-------------|
| `--form` | `-f` | Skip checking forms |
| `--module` | `-m` | Skip checking modules |
| `--class` | `-c` | Skip checking classes |
| `--reference` | `-r` | Skip checking references |

### Examples

```sh
# Check all projects in the current directory
aspen check

# Check a specific project
aspen check path/to/project.vbp

# Check a directory, skipping form validation
aspen check --form path/to/projects/
```

## Installation

```sh
cargo install --path projects/aspen
```

Or from the workspace root:

```sh
cargo build -p aspen
```
