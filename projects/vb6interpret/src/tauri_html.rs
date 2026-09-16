//! Embedded HTML content for the Tauri webview.
//!
//! This module provides a const string containing the full HTML page served
//! to the Tauri webview when running a VB6 form application. It includes
//! inline CSS and JavaScript for communicating with the Rust backend via
//! Tauri IPC commands.

#[cfg(feature = "tauri")]
#[allow(dead_code)]

/// The embedded HTML page shown in the Tauri webview.
pub const HTML: &str = r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>VB6 Form Viewer</title>
    <style>
        body { margin: 0; padding: 10px; background: #2d2d2d; color: #fff; font-family: sans-serif; }
        #root { min-height: 200px; background: #1e1e1e; padding: 10px; border: 1px solid #444; }
        #status { color: #888; font-size: 12px; margin-top: 10px; }
        .vb-control { /* VB6 form control styling */ }
    </style>
</head>
<body>
    <div id="root"></div>
    <div id="status">Ready</div>
    <script type="module">
        import { invoke } from '@tauri-apps/api/core';

        window.updateForm = async function(handle) {
            try {
                const html = await invoke('update_form', { handle });
                window.rootEl.innerHTML = html;
                window.statusEl.textContent = 'Form updated';
            } catch (e) {
                window.statusEl.textContent = 'Update error: ' + e;
            }
        };

        window.rootEl = document.getElementById('root');
        window.statusEl = document.getElementById('status');

        new MutationObserver(() => {
            if (window.rootEl.innerHTML.trim()) {
                window.statusEl.textContent = 'Form loaded';
            }
        }).observe(window.rootEl, { childList: true, subtree: true, characterData: true, attributes: true });
    </script>
</body>
</html>"#;
