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

        // Attach DOM event listeners to rendered form controls based on the
        // event bindings provided by the Rust backend.
        //
        // The `bindings` array contains objects with {control, event, procedure}
        // fields. For each binding, this function finds the DOM element whose
        // id matches the control name and attaches a listener for the specified
        // event that dispatches to the Tauri IPC handler.
        window.attachFormEvents = async function(engineHandle, formName, bindings) {
            if (!bindings || bindings.length === 0) {
                window.statusEl.textContent = 'No event bindings to attach';
                return;
            }

            for (const binding of bindings) {
                const { control, event, procedure } = binding;
                // The Tauri renderer uses id="controlName" for all controls
                const el = document.getElementById(control);
                if (el) {
                    el.addEventListener(event, async (e) => {
                        try {
                            const status = await invoke('form_event', {
                                engineHandle: engineHandle,
                                control: control,
                                event: event,
                            });
                            if (status === 'Handled') {
                                window.statusEl.textContent = 'Event dispatched: ' + procedure;
                            } else if (status === 'Terminated') {
                                window.statusEl.textContent = 'Program terminated';
                            }
                        } catch (err) {
                            window.statusEl.textContent = 'Event error: ' + err;
                        }
                    });
                }
            }

            window.statusEl.textContent = 'Events attached: ' + bindings.length + ' binding(s)';
        };

        // Auto-attach event bindings after the form HTML is loaded.
        // This is triggered by the Rust side setting window.__vb6FormName__.
        window.attachFormEventsAutomatically = async function(engineHandle) {
            if (!window.__vb6FormName__) {
                return;
            }

            try {
                const bindings = await invoke('form_event_bindings', {
                    engineHandle: engineHandle,
                    formName: window.__vb6FormName__,
                });
                await window.attachFormEvents(engineHandle, window.__vb6FormName__, bindings);
            } catch (err) {
                window.statusEl.textContent = 'Event attach error: ' + err;
            }
        };

        window.rootEl = document.getElementById('root');
        window.statusEl = document.getElementById('status');

        new MutationObserver(() => {
            if (window.rootEl.innerHTML.trim()) {
                window.statusEl.textContent = 'Form loaded';
                // After first render, try to auto-attach event bindings.
                // This runs once when the engine handle is available.
                if (window.__vb6EngineHandle__ !== undefined) {
                    window.attachFormEventsAutomatically(window.__vb6EngineHandle__);
                }
            }
        }).observe(window.rootEl, { childList: true, subtree: true, characterData: true, attributes: true });
    </script>
</body>
</html>"#;
