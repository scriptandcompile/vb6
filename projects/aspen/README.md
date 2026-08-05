# Aspen

Aspen is a VB6 analysis tool in the spirit of `cargo check`, `cargo fmt`, etc.

## Commands

### `check`

Validate one or more VB6 projects with semantic analysis.

```
aspen check [project path]
```

If `[project path]` is a directory, Aspen recursively searches for `.vbp` files
and checks each one. If it points to a single `.vbp` file, only that project is
checked. Defaults to the current directory.

Each project's references, missing source files, and syntax are validated, then
the project is analyzed semantically (symbol tables, name resolution, and
duplicate definitions). Errors, warnings, and missing files are reported per
project.

### `fmt`

Reformat VB6 source files (`.bas`, `.cls`, `.frm`) with correct block indentation.

```
aspen fmt [OPTIONS] [project path]
```

If `[project path]` is a directory, Aspen recursively discovers VB6 source files
or `.vbp` project files. Defaults to the current directory.

| Flag | Description |
|------|-------------|
| `--check` | Only check formatting; exit 1 if any file would change |
| `--keyword`, `-K` | Format VB6 keywords with one of: `upper`, `lower`, `camel`, `first` |
| `--indent-size` | Spaces per indent level (default: 4) |
| `--blank-lines-around-directives` | Insert blank line before `#If` and after `#End If` |
| `--blank-lines-inside-directives` | Insert blank lines between `#If`/`#ElseIf`/`#Else`/`#End If` and their bodies |
| `--blank-lines-around-top-level` | Insert a blank line between procedures and other top-level constructs while keeping preceding comment lines attached to the following construct |

Settings can also be specified in `.aspen.toml`:

```toml
[fmt]
keyword_case = "camel" # one of: "upper", "lower", "camel", "first"
indent_size = 4
blank_lines_around_directives = true
blank_lines_inside_directives = false
blank_lines_around_top_level = true
```

### Examples

```sh
# Check all projects in the current directory
aspen check

# Check a specific project
aspen check path/to/project.vbp

# Format a single file
aspen fmt path/to/source.bas

# Check if files are formatted without writing
aspen fmt --check path/to/project.vbp

# Format keywords in uppercase
aspen fmt --keyword upper

# Format with 2-space indentation and blank lines around directives
aspen fmt --indent-size 2 --blank-lines-around-directives

# Add spacing before procedures and other top-level constructs
aspen fmt --blank-lines-around-top-level
```

## Installation

### From source

```sh
cargo install --path projects/aspen
```

Or from the workspace root:

```sh
cargo build -p aspen
```

### Windows release artifact

For each new Aspen release, the GitHub Actions workflow builds both a Windows zip archive and an MSI installer and publishes them as release assets. The packaged directory contains:

- `bin/aspen.exe`
- `bin/aspen.cmd`
- `bin/aspen.ps1`
- `README.txt`

To use the zip bundle from a normal Windows terminal, add the `bin` directory to your `PATH` and run `aspen`.

Example:

```powershell
$aspenDir = "C:\path\to\aspen"
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";$aspenDir\bin", "User")
```

For the MSI installer, run the downloaded `.msi` file and follow the prompts. The installer adds Aspen to the system `PATH`, so after installation you can run:

```powershell
aspen --help
```
