//! Embedded HTML content for the Tauri webview.
//!
//! This module builds the full HTML page served to the Tauri webview when
//! running a VB6 form application. The page is served over a registered
//! custom URI scheme protocol (see `main.rs::launch_tauri`), which lets the
//! webview load a real document instead of an `about:blank` shell:
//!
//! - The page gets a proper origin (e.g. `vb6://localhost`), so Tauri
//!   classifies it as a local origin and injects its IPC init scripts.
//! - The VB6 form HTML is present in the document before any script runs, so
//!   the window is never blank.
//! - IPC uses `window.__TAURI_INTERNALS__.invoke`, which is injected into
//!   every webview unconditionally (it does not depend on `withGlobalTauri`).

#[cfg(feature = "tauri")]
#[allow(dead_code)]
/// Build the complete HTML page shown in the Tauri webview for a form project.
///
/// `form_html` is the rendered VB6 control markup (injected directly into
/// `#root` so the form is visible before any script runs), `css` is the bare
/// VB6 stylesheet, and `form_name`/`engine_handle` are exposed to the inline
/// script so it can auto-attach event bindings and dispatch events over IPC.
pub fn build_page(form_html: &str, css: &str, form_name: &str, engine_handle: u32) -> String {
    let form_name_json = serde_json::to_string(form_name).unwrap_or_else(|_| "\"\"".into());
    let engine_handle = engine_handle.to_string();

    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>VB6Interpret</title>
    <style>
{css}
        html, body {{ height: 100%; }}
    </style>
</head>
<body>
    <div id="root">{form_html}</div>
    <script>
    (function () {{
        'use strict';

        var invoke = function (cmd, args) {{
            var internals = window.__TAURI_INTERNALS__;
            return internals ? internals.invoke(cmd, args || {{}}) : Promise.reject('__TAURI_INTERNALS__ missing');
        }};

        window.__vb6FormName__ = {form_name_json};
        window.__vb6EngineHandle__ = {engine_handle};

        // updateForm: re-render a form by handle.
        window.updateForm = function (handle) {{
            return invoke('update_form', {{ handle: handle }})
                .then(function (html) {{
                    document.getElementById('root').innerHTML = html;
                    return window._vb6AutoAttach();
                }})
                .catch(function (e) {{
                    console.error('Update error:', e);
                }});
        }};

        // attachFormEvents: attach DOM event listeners to rendered controls.
        window.attachFormEvents = function (engineHandle, formName, bindings) {{
            if (!bindings || bindings.length === 0) return;
            bindings.forEach(function (binding) {{
                var el = document.getElementById(binding.control);
                if (!el) return;
                // Re-attaching (e.g. after a re-render) must not stack duplicate
                // listeners on the same element.
                var key = '__vb6Handler_' + binding.procedure;
                if (el[key]) {{
                    el.removeEventListener(binding.event, el[key]);
                    delete el[key];
                }}
                el[key] = function () {{
                    if (el.disabled) return;
                    invoke('form_event', {{
                        engineHandle: engineHandle,
                        control: binding.control,
                        event: binding.event
                    }})
                        .then(function (result) {{
                            if (result !== 'Handled') console.log('Program terminated');
                        }})
                        .catch(function (err) {{
                            console.error('Event error:', err);
                        }});
                }};
                el.addEventListener(binding.event, el[key]);
            }});
        }};

        // Auto-attach event bindings once form name and engine handle are set.
        window._vb6AutoAttach = function () {{
            if (!window.__vb6FormName__ || window.__vb6EngineHandle__ === undefined) return;
            return invoke('form_event_bindings', {{
                engineHandle: window.__vb6EngineHandle__,
                formName: window.__vb6FormName__
            }})
                .then(function (bindings) {{
                    return window.attachFormEvents(
                        window.__vb6EngineHandle__,
                        window.__vb6FormName__,
                        bindings
                    );
                }})
                .catch(function (err) {{
                    console.error('Event attach error:', err);
                }});
        }};

        // Start the project (merges procedures and fires Form_Load), then wire
        // up control event bindings. run_project resolves only after startup
        // completes, so handlers exist before any user interaction.
        window._vb6StartProject = function () {{
            return invoke('run_project', {{ engineHandle: window.__vb6EngineHandle__ }})
                .then(function (res) {{
                    if (!res) throw new Error('engine not found');
                    if (!res.started) throw new Error(res.error || 'project failed to start');
                }})
                .catch(function (err) {{
                    console.error('Startup error:', err);
                }})
                .then(function () {{
                    return window._vb6AutoAttach();
                }});
        }};

        window._vb6StartProject();
    }})();
    </script>
</body>
</html>"#
    )
}
