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
/// `<body>` so the form is visible before any script runs), `css` is the bare
/// VB6 stylesheet, and `form_name`/`engine_handle`/`form_handle` are exposed
/// to the inline script so it can auto-attach event bindings and dispatch
/// events over IPC. The form handle is used to look up the form's natural
/// dimensions for sizing the body.
pub fn build_page(
    form_html: &str,
    css: &str,
    form_name: &str,
    engine_handle: u32,
    form_handle: u32,
) -> String {
    let form_name_json = serde_json::to_string(form_name).unwrap_or_else(|_| "\"\"".into());
    let engine_handle = engine_handle.to_string();

    let (fw, fh) = vb6runtime::layout::renderer::TauriRenderer::form_dimensions(form_handle);
    let body_style = if fw > 0.0 || fh > 0.0 {
        format!(
            " style=\"{}{}{}{}\"",
            if fw > 0.0 {
                format!("width:{fw:.1}px;")
            } else {
                String::new()
            },
            if fw > 0.0 { "" } else { "width:100%;" },
            if fh > 0.0 {
                format!("height:{fh:.1}px;")
            } else {
                String::new()
            },
            if fh > 0.0 { "" } else { "height:100%;" }
        )
    } else {
        " style=\"width:100%;height:100%;\"".to_string()
    };

    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>VB6Interpret</title>
    <style>
{css}
    </style>
</head>
<body{body_style}>{form_html}</body>
    <script>
    (function () {{
        'use strict';

        var invoke = function (cmd, args) {{
            var internals = window.__TAURI_INTERNALS__;
            return internals ? internals.invoke(cmd, args || {{}}) : Promise.reject('__TAURI_INTERNALS__ missing');
        }};

        window.__vb6FormName__ = {form_name_json};
        window.__vb6EngineHandle__ = {engine_handle};
        window.__vb6FormHandle__ = {form_handle};

        // updateForm: re-render a form by handle.
        window.updateForm = function (handle) {{
            return invoke('update_form', {{ handle: handle }})
                .then(function (html) {{
                    document.body.innerHTML = html;
                    return window._vb6AutoAttach();
                }})
                .catch(function (e) {{
                    console.error('Update error:', e);
                }});
        }};

        // attachFormEvents: attach DOM event listeners to rendered controls.
        window.attachFormEvents = function (engineHandle, formHandle, bindings) {{
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

        // Auto-attach event bindings once form and engine are set.
        window._vb6AutoAttach = function () {{
            if (window.__vb6FormHandle__ === undefined) return;
            return invoke('form_event_bindings', {{
                formHandle: window.__vb6FormHandle__
            }})
                .then(function (bindings) {{
                    return window.attachFormEvents(
                        window.__vb6EngineHandle__,
                        window.__vb6FormHandle__,
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
