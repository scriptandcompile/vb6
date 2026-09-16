//! Menubar and context/popup menu rendering.
//!
//! Produces platform-specific output: native Tauri menus or HTML/CSS-styled divs for WASM.
//!
//! # Menubar Rendering
//!
//! On WASM, menus are rendered as HTML divs with CSS styling:
//! - `<div class="vb6-menubar">` — container for all menus
//! - `<div class="vb6-menu">` — individual menu
//! - `<ul class="vb6-menu-items">` — dropdown items
//! - `<li class="vb6-menu-item">` — individual menu items
//!
//! On Tauri, menus use the native OS menu bar via `tauri::menu::Menu`.
//!
//! # Context Menu Rendering
//!
//! On WASM, context/popup menus are rendered as absolutely positioned divs:
//! - `<div class="vb6-popup-menu">` — positioned at cursor coordinates
//!
//! On Tauri, context menus use the native `tauri::menu::Submenu` API.

#[cfg(feature = "tauri")]
use tauri::menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder};
#[cfg(feature = "tauri")]
use tauri::{Manager, Wry};

use vb6parse::language::MenuControl;

/// Build HTML for a menubar from a list of VB6 [`MenuControl`] entries.
///
/// Produces a self-contained HTML fragment with proper CSS class names
/// and HTML-escaped captions.
///
/// # Examples
/// ```
/// use vb6parse::language::{MenuControl, MenuProperties};
/// use vb6runtime::layout::menu::build_menu_html;
///
/// let menus = vec![
///     MenuControl::new(
///         "File".into(), String::new(), 0,
///         MenuProperties { caption: "File".into(), ..Default::default() },
///         vec![],
///     ),
/// ];
/// let html = build_menu_html(&menus);
/// assert!(html.contains("vb6-menubar"));
/// assert!(html.contains("vb6-menu"));
/// ```
pub fn build_menu_html(menus: &[MenuControl]) -> String {
    let mut html = String::from("<div class=\"vb6-menubar\">");

    for menu in menus {
        html.push_str(&format!(
            r#"<div class="vb6-menu" id="{}">"#,
            html_escape(menu.name())
        ));
        html.push_str(&format!(
            "<div class=\"vb6-menu-caption\">{}</div>",
            html_escape(&menu.properties().caption)
        ));
        html.push_str("<ul class=\"vb6-menu-items\">");

        for sub in menu.sub_menus() {
            html.push_str(&format!(
                r#"<li class="vb6-menu-item" id="{}">{}</li>"#,
                html_escape(sub.name()),
                html_escape(&sub.properties().caption)
            ));
        }

        html.push_str("</ul></div>");
    }

    html.push_str("</div>");
    html
}

/// Build HTML for a context (popup) menu by name.
///
/// Returns `None` if no menu with the given name is found.
///
/// The returned HTML is a single absolutely positioned div that can be
/// placed at cursor coordinates via JavaScript event handlers.
///
/// # Examples
/// ```
/// use vb6parse::language::{MenuControl, MenuProperties};
/// use vb6runtime::layout::menu::build_context_html;
///
/// let menus = vec![
///     MenuControl::new(
///         "ContextMenu1".into(), String::new(), 0,
///         MenuProperties { caption: "Context".into(), ..Default::default() },
///         vec![],
///     ),
/// ];
/// assert!(build_context_html(&menus, "ContextMenu1").is_some());
/// assert!(build_context_html(&menus, "NoSuchMenu").is_none());
/// ```
pub fn build_context_html(menus: &[MenuControl], menu_name: &str) -> Option<String> {
    let vb_menu = menus.iter().find(|m| m.name() == menu_name)?;

    let mut html = String::from("<div class=\"vb6-popup-menu\" id=\"ctxmenu\">");

    for sub in vb_menu.sub_menus() {
        html.push_str(&format!(
            r#"<div class="vb6-menu-item" id="{}">{}</div>"#,
            html_escape(sub.name()),
            html_escape(&sub.properties().caption)
        ));
    }

    html.push_str("</div>");
    Some(html)
}

/// Build a native Tauri menubar from a list of VB6 [`MenuControl`] entries.
///
/// Creates a [`Menu`] with one [`tauri::menu::MenuItem`] per top-level menu.
/// Sub-menus are currently not included.
///
/// # Tauri Integration
///
/// The returned menu should be set on the window via:
/// ```ignore
/// // In Tauri main.rs setup handler
/// let menu = build_tauri_menus(app.handle(), &menus)?;
/// app.set_menu(menu)?;
/// ```
///
/// # Examples
/// ```
/// # #[cfg(feature = "tauri")]
/// # {
/// use vb6parse::language::{MenuControl, MenuProperties};
/// use vb6runtime::layout::menu::build_tauri_menus;
///
/// let menus = vec![
///     MenuControl::new(
///         "File".into(), String::new(), 0,
///         MenuProperties { caption: "&File".into(), ..Default::default() },
///         vec![],
///     ),
///     MenuControl::new(
///         "Edit".into(), String::new(), 0,
///         MenuProperties { caption: "&Edit".into(), ..Default::default() },
///         vec![],
///     ),
/// ];
/// // Would be called with an app handle in real code:
/// // let menu = build_tauri_menus(app.handle(), &menus);
/// # }
/// ```
#[cfg(feature = "tauri")]
pub fn build_tauri_menus<M: Manager<Wry>>(
    manager: &M,
    menus: &[MenuControl],
) -> tauri::Result<tauri::menu::Menu<Wry>> {
    let items: Vec<tauri::menu::MenuItem<Wry>> = menus
        .iter()
        .map(|m| {
            MenuItemBuilder::new(&m.properties().caption)
                .enabled(true)
                .build(manager)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let menu = MenuBuilder::new(manager).build()?;
    for item in items {
        menu.append(&item)?;
    }
    Ok(menu)
}

/// Build a native Tauri context menu by name.
///
/// Returns `None` if no menu with the given name is found.
///
/// The returned [`Submenu`] can be shown as a context menu via
/// `window.menu_handle().get(&id)?.popup(window)` or `submenu.popup(window)`.
///
/// # Tauri Integration
///
/// ```ignore
/// // Listen for contextmenu event in JS, then:
/// if let Some(submenu) = build_context_menu(app.handle(), &menus, "ContextMenu1")? {
///     submenu.popup(window)?;
/// }
/// ```
///
/// # Examples
/// ```
/// # #[cfg(feature = "tauri")]
/// # {
/// use vb6parse::language::{MenuControl, MenuProperties};
/// use vb6runtime::layout::menu::build_context_menu;
///
/// let menus = vec![
///     MenuControl::new(
///         "File".into(), String::new(), 0,
///         MenuProperties { caption: "File".into(), ..Default::default() },
///         vec![],
///     ),
///     MenuControl::new(
///         "ContextMenu1".into(), String::new(), 0,
///         MenuProperties { caption: "Context".into(), ..Default::default() },
///         vec![],
///     ),
/// ];
/// // Would be called with an app handle in real code:
/// // let ctx = build_context_menu(app.handle(), &menus, "ContextMenu1");
/// # }
/// ```
#[cfg(feature = "tauri")]
pub fn build_context_menu<M: Manager<Wry>>(
    manager: &M,
    menus: &[MenuControl],
    menu_name: &str,
) -> tauri::Result<Option<tauri::menu::Submenu<Wry>>> {
    let vb_menu = menus
        .iter()
        .find(|m| m.name() == menu_name)
        .ok_or_else(|| anyhow::anyhow!("menu '{}' not found", menu_name))?;

    let submenu = SubmenuBuilder::new(manager, &vb_menu.properties().caption).build()?;

    for sub in vb_menu.sub_menus() {
        let item = MenuItemBuilder::new(&sub.properties().caption)
            .enabled(true)
            .build(manager)?;
        submenu.append(&item)?;
    }

    Ok(Some(submenu))
}

/// Escape special HTML characters in a string.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use vb6parse::language::{MenuControl, MenuProperties};

    fn sample_menu(name: &str, caption: &str) -> MenuControl {
        MenuControl::new(
            name.into(),
            String::new(),
            0,
            MenuProperties {
                caption: caption.into(),
                ..Default::default()
            },
            vec![],
        )
    }

    fn sample_menu_with_items(name: &str, caption: &str, items: Vec<(&str, &str)>) -> MenuControl {
        MenuControl::new(
            name.into(),
            String::new(),
            0,
            MenuProperties {
                caption: caption.into(),
                ..Default::default()
            },
            items
                .into_iter()
                .map(|(n, c)| MenuControl::new(
                    n.into(),
                    String::new(),
                    0,
                    MenuProperties {
                        caption: c.into(),
                        ..Default::default()
                    },
                    vec![],
                ))
                .collect(),
        )
    }

    #[test]
    fn menu_html_structure() {
        let menus = vec![sample_menu("File", "File")];
        let html = build_menu_html(&menus);
        assert_eq!(
            html,
            "<div class=\"vb6-menubar\"><div class=\"vb6-menu\" id=\"File\">\
             <div class=\"vb6-menu-caption\">File</div>\
             <ul class=\"vb6-menu-items\"></ul></div></div>"
        );
    }

    #[test]
    fn menu_html_with_items() {
        let menus = vec![sample_menu_with_items(
            "File",
            "File",
            vec![("Open", "Open"), ("Exit", "E_xit")],
        )];
        let html = build_menu_html(&menus);
        assert_eq!(
            html,
            "<div class=\"vb6-menubar\"><div class=\"vb6-menu\" id=\"File\">\
             <div class=\"vb6-menu-caption\">File</div>\
             <ul class=\"vb6-menu-items\">\
             <li class=\"vb6-menu-item\" id=\"Open\">Open</li>\
             <li class=\"vb6-menu-item\" id=\"Exit\">E_xit</li>\
             </ul></div></div>"
        );
    }

    #[test]
    fn menu_html_escapes_captions() {
        let menus = vec![sample_menu("File", "File&Open")];
        let html = build_menu_html(&menus);
        assert_eq!(
            html,
            "<div class=\"vb6-menubar\"><div class=\"vb6-menu\" id=\"File\">\
             <div class=\"vb6-menu-caption\">File&amp;Open</div>\
             <ul class=\"vb6-menu-items\"></ul></div></div>"
        );
    }

    #[test]
    fn context_html_structure() {
        let menus = vec![sample_menu_with_items(
            "ContextMenu1",
            "Context",
            vec![("Cut", "Cut"), ("Copy", "Copy")],
        )];
        let html = build_context_html(&menus, "ContextMenu1").unwrap();
        assert_eq!(
            html,
            "<div class=\"vb6-popup-menu\" id=\"ctxmenu\">\
             <div class=\"vb6-menu-item\" id=\"Cut\">Cut</div>\
             <div class=\"vb6-menu-item\" id=\"Copy\">Copy</div>\
             </div>"
        );
    }

    #[test]
    fn context_menu_by_name() {
        let menus = vec![
            sample_menu("File", "File"),
            sample_menu("ContextMenu1", "Context"),
        ];
        assert!(build_context_html(&menus, "ContextMenu1").is_some());
        assert!(build_context_html(&menus, "File").is_some());
        assert!(build_context_html(&menus, "NoSuchMenu").is_none());
    }

    #[test]
    fn empty_menu_list() {
        let html = build_menu_html(&[]);
        assert_eq!(html, "<div class=\"vb6-menubar\"></div>");
    }

    #[test]
    fn menu_with_special_characters() {
        let menus = vec![sample_menu_with_items(
            "Menu",
            "Menu <script>",
            vec![("Item & Item", "Item <b>")],
        )];
        let html = build_menu_html(&menus);
        assert_eq!(
            html,
            "<div class=\"vb6-menubar\"><div class=\"vb6-menu\" id=\"Menu\">\
             <div class=\"vb6-menu-caption\">Menu &lt;script&gt;</div>\
             <ul class=\"vb6-menu-items\">\
             <li class=\"vb6-menu-item\" id=\"Item &amp; Item\">Item &lt;b&gt;</li>\
             </ul></div></div>"
        );
    }
}
