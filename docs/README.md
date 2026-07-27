# VB6 Workspace Docs

This directory is the umbrella GitHub Pages scaffold for the full VB6 workspace.

## Pages

- `index.html` - workspace hub and project launcher
- `status.html` - status board and remaining work
- `vb6parse/` - VB6Parse project docs (library reference, benchmarks, coverage, playground)

## Local Preview

Because the hub loads project metadata from `assets/data/projects.json`, serve the directory over HTTP instead of opening the file directly:

```bash
python3 -m http.server --directory docs 8000
```

Then open `http://localhost:8000/` in a browser.

## Deployment

The root workflow deploys the `docs/` directory to GitHub Pages.

If you add or rename projects, update `assets/data/projects.json` and the hub/status pages will pick up the change automatically.