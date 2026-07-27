#!/usr/bin/env python3
"""
Documentation Bundler - Converts multiple Markdown files into a single PDF.

This script bundles various markdown documentation files from the VB6 project
into a single, well-structured PDF document.

Dependencies:
    # Using virtual environment (recommended):
    source .venv/bin/activate
    pip install markdown weasyprint pymdown-extensions pygments
    
    # Or using requirements file:
    pip install -r requirements-docs.txt

Usage:
    python bundle_docs.py [output.pdf]
    # Or: .venv/bin/python bundle_docs.py [output.pdf]
    
    To customize which files are included, edit the MARKDOWN_FILES list below.
"""

import os
import sys
from pathlib import Path
from datetime import datetime
import markdown
from weasyprint import HTML, CSS
from weasyprint.text.fonts import FontConfiguration

# Base directory for the project
BASE_DIR = Path(__file__).parent.resolve()

# List of markdown files to include in the PDF (relative to BASE_DIR)
# Add or remove files from this list as needed
MARKDOWN_FILES = [
    # Root documentation
    "README.md",
    
    # Plans and architecture
    "plans/IMPLEMENTATION_PLAN.md",
    "plans/LIBRARY_ARCHITECTURE.md",
    "plans/CODEGEN_EXTRACTION.md",
    "plans/semantic_analysis.md",
    "plans/TEST_HARNESS.md",
    "plans/playground.md",
    
    # Component READMEs
    "projects/aspen/README.md",
    "projects/vb6parse/README.md",
    "projects/vb6core/README.md",
    "projects/vb6semantic/README.md",
    "projects/vb6codegen/README.md",
    "projects/vb6compile/README.md",
    "projects/vb6convert/README.md",
    "projects/vb6interpret/README.md",
    "projects/vb6libraries/README.md",
    "projects/vb6runtime/README.md",
    
    # Design documents
    "projects/vb6core/docs/DESIGN.md",
    "projects/vb6codegen/docs/DESIGN.md",
    "projects/vb6codegen/docs/FEATURE_GRID.md",
    "projects/vb6compile/docs/DESIGN.md",
    "projects/vb6runtime/docs/DESIGN.md",
    
    # Convert-specific documentation
    "projects/vb6convert/docs/GETTING_STARTED.md",
    "projects/vb6convert/docs/ARCHITECTURE.md",
    "projects/vb6convert/docs/ROADMAP.md",
    "projects/vb6convert/docs/IMPLEMENTATION_GUIDE.md",
    "projects/vb6convert/docs/TESTING.md",
    
    # Libraries documentation
    "projects/vb6libraries/docs/CROSS_CUTTING_EXAMPLE.md",
    
    # Parse-specific documentation
    "projects/vb6parse/CHANGELOG.md",
    "projects/vb6parse/CONTRIBUTING.md",
]

# CSS styling for the PDF
PDF_STYLE = """
@page {
    size: A4;
    margin: 2.5cm;
    @top-right {
        content: counter(page);
        font-size: 10pt;
        color: #666;
    }
}

body {
    font-family: 'DejaVu Sans', Arial, sans-serif;
    font-size: 11pt;
    line-height: 1.6;
    color: #333;
}

h1 {
    color: #2c3e50;
    font-size: 24pt;
    margin-top: 1.5em;
    margin-bottom: 0.5em;
    page-break-after: avoid;
    border-bottom: 2px solid #3498db;
    padding-bottom: 0.3em;
}

h2 {
    color: #34495e;
    font-size: 18pt;
    margin-top: 1.2em;
    margin-bottom: 0.4em;
    page-break-after: avoid;
}

h3 {
    color: #34495e;
    font-size: 14pt;
    margin-top: 1em;
    margin-bottom: 0.3em;
    page-break-after: avoid;
}

h4, h5, h6 {
    color: #555;
    margin-top: 0.8em;
    margin-bottom: 0.3em;
    page-break-after: avoid;
}

.file-header {
    background-color: #ecf0f1;
    padding: 1em;
    margin: 2em 0 1em 0;
    border-left: 4px solid #3498db;
    page-break-before: always;
}

.file-header h1 {
    margin: 0;
    border-bottom: none;
    font-size: 16pt;
    color: #2c3e50;
}

.file-path {
    font-family: 'DejaVu Sans Mono', 'Courier New', monospace;
    font-size: 9pt;
    color: #7f8c8d;
    margin-top: 0.3em;
}

code {
    font-family: 'DejaVu Sans Mono', 'Courier New', monospace;
    font-size: 9pt;
    background-color: #f8f9fa;
    padding: 0.2em 0.4em;
    border-radius: 3px;
    color: #e74c3c;
}

pre {
    background-color: #f8f9fa;
    border: 1px solid #dee2e6;
    border-radius: 4px;
    padding: 1em;
    overflow-x: auto;
    page-break-inside: avoid;
    font-size: 9pt;
}

pre code {
    background-color: transparent;
    padding: 0;
    color: inherit;
}

blockquote {
    border-left: 4px solid #3498db;
    padding-left: 1em;
    margin-left: 0;
    color: #555;
    font-style: italic;
    page-break-inside: avoid;
}

table {
    border-collapse: collapse;
    width: 100%;
    margin: 1em 0;
    page-break-inside: avoid;
}

th, td {
    border: 1px solid #ddd;
    padding: 0.5em;
    text-align: left;
}

th {
    background-color: #3498db;
    color: white;
    font-weight: bold;
}

tr:nth-child(even) {
    background-color: #f8f9fa;
}

a {
    color: #3498db;
    text-decoration: none;
}

ul, ol {
    margin: 0.5em 0;
    padding-left: 2em;
}

li {
    margin: 0.3em 0;
}

.toc {
    page-break-after: always;
    margin-bottom: 2em;
}

.toc h1 {
    margin-bottom: 1em;
}

.toc ul {
    list-style-type: none;
    padding-left: 1em;
}

.toc li {
    margin: 0.4em 0;
}

.cover-page {
    text-align: center;
    page-break-after: always;
    padding-top: 5cm;
}

.cover-page h1 {
    font-size: 36pt;
    margin-bottom: 0.5em;
    border-bottom: none;
}

.cover-page .subtitle {
    font-size: 18pt;
    color: #7f8c8d;
    margin-bottom: 2em;
}

.cover-page .date {
    font-size: 12pt;
    color: #95a5a6;
    margin-top: 3em;
}
"""


def read_markdown_file(file_path: Path) -> str:
    """Read a markdown file and return its content."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            return f.read()
    except FileNotFoundError:
        print(f"Warning: File not found: {file_path}")
        return f"# File Not Found\n\n**{file_path}** could not be read."
    except Exception as e:
        print(f"Warning: Error reading {file_path}: {e}")
        return f"# Error Reading File\n\n**{file_path}** encountered an error: {e}"


def markdown_to_html(md_content: str) -> str:
    """Convert markdown content to HTML."""
    md = markdown.Markdown(extensions=[
        'extra',           # Tables, code blocks, etc.
        'codehilite',      # Syntax highlighting
        'toc',             # Table of contents
        'fenced_code',     # Fenced code blocks
        'tables',          # Tables support
        'nl2br',           # Newline to <br>
    ])
    return md.convert(md_content)


def create_cover_page() -> str:
    """Create a cover page for the PDF."""
    date_str = datetime.now().strftime("%B %d, %Y")
    return f"""
    <div class="cover-page">
        <h1>VB6 Project Documentation</h1>
        <div class="subtitle">Complete Technical Documentation Bundle</div>
        <div class="date">Generated on {date_str}</div>
    </div>
    """


def create_toc(files: list) -> str:
    """Create a table of contents."""
    toc_html = '<div class="toc"><h1>Table of Contents</h1><ul>'
    
    for file_path in files:
        file_name = Path(file_path).name
        section_name = file_path.replace('.md', '').replace('_', ' ').replace('-', ' ')
        toc_html += f'<li>{section_name} ({file_name})</li>'
    
    toc_html += '</ul></div>'
    return toc_html


def bundle_markdown_files(output_pdf: str = "vb6_documentation.pdf"):
    """
    Bundle all markdown files into a single PDF.
    
    Args:
        output_pdf: Name of the output PDF file (default: vb6_documentation.pdf)
    """
    print("VB6 Documentation Bundler")
    print("=" * 60)
    
    # Collect and convert all markdown files
    all_html_content = []
    
    # Add cover page
    all_html_content.append(create_cover_page())
    
    # Add table of contents
    existing_files = [f for f in MARKDOWN_FILES if (BASE_DIR / f).exists()]
    all_html_content.append(create_toc(existing_files))
    
    # Process each markdown file
    for md_file in MARKDOWN_FILES:
        file_path = BASE_DIR / md_file
        
        if not file_path.exists():
            print(f"⚠ Skipping missing file: {md_file}")
            continue
        
        print(f"📄 Processing: {md_file}")
        
        # Read markdown content
        md_content = read_markdown_file(file_path)
        
        # Convert to HTML
        html_content = markdown_to_html(md_content)
        
        # Add file header
        file_header = f"""
        <div class="file-header">
            <h1>{file_path.stem}</h1>
            <div class="file-path">{md_file}</div>
        </div>
        """
        
        all_html_content.append(file_header)
        all_html_content.append(html_content)
    
    # Combine all HTML
    full_html = f"""
    <!DOCTYPE html>
    <html>
    <head>
        <meta charset="utf-8">
        <title>VB6 Project Documentation</title>
    </head>
    <body>
        {''.join(all_html_content)}
    </body>
    </html>
    """
    
    # Generate PDF
    print("\n📦 Generating PDF...")
    output_path = BASE_DIR / output_pdf
    
    font_config = FontConfiguration()
    html_obj = HTML(string=full_html, base_url=str(BASE_DIR))
    css = CSS(string=PDF_STYLE, font_config=font_config)
    
    html_obj.write_pdf(
        output_path,
        stylesheets=[css],
        font_config=font_config
    )
    
    print(f"\n✓ PDF generated successfully: {output_path}")
    print(f"  Size: {output_path.stat().st_size / 1024:.2f} KB")
    print(f"  Files included: {len(existing_files)}")


def main():
    """Main entry point."""
    output_file = sys.argv[1] if len(sys.argv) > 1 else "vb6_documentation.pdf"
    
    try:
        bundle_markdown_files(output_file)
    except Exception as e:
        print(f"\n❌ Error: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    main()
