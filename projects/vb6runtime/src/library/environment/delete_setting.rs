//! VB6 DeleteSetting statement syntax:
//!
//! ```vb
//! DeleteSetting appname, section[, key]
//! ```
//!
//! Deletes a section or key setting from an application's entry in the Windows registry.
//!
//! The DeleteSetting statement syntax has these named arguments:
//!
//! | Part     | Description |
//! |----------|-------------|
//! | appname  | Required. String expression containing the name of the application or project to which the section or key setting applies. |
//! | section  | Required. String expression containing the name of the section from which the key setting is being deleted. If only appname and section are provided, the specified section is deleted along with all related key settings. |
//! | key      | Optional. String expression containing the name of the key setting being deleted. |
//!
//! Examples:
//! - DeleteSetting "MyApp", "Startup" (deletes entire Startup section)
//! - DeleteSetting "MyApp", "Startup", "Left" (deletes Left key from Startup section)
//! - DeleteSetting App.ProductName, "FileFilter" (deletes FileFilter section)
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/deletesetting-statement)

use crate::error::VBResult;
use crate::state::settings;
use crate::value::{VBString, VBVariant};

/// Deletes a section or key setting from the VB6 settings store.
///
/// When `key` is `None` or names no key (empty), the entire section is
/// removed; otherwise only that key is removed. This deliberately preserves
/// the historical conflation of an omitted key with an explicit empty one.
pub fn delete_setting(
    appname: &VBString,
    section: &VBString,
    key: Option<&VBString>,
) -> VBResult<VBVariant> {
    match key {
        Some(k) if !k.as_str().is_empty() => {
            settings::remove_key(appname.as_str(), section.as_str(), k.as_str())
                .map_err(|e| crate::error::VBError::with_description(5, e.to_string()))?;
        }
        _ => {
            settings::remove_section(appname.as_str(), section.as_str())
                .map_err(|e| crate::error::VBError::with_description(5, e.to_string()))?;
        }
    }
    Ok(VBVariant::Empty)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::settings as settings_state;
    use crate::state::test_support::with_temp_settings_store;

    #[test]
    fn deletes_a_specific_key() {
        with_temp_settings_store(|_| {
            settings_state::set("MyApp", "Window", "Left", "150").unwrap();
            settings_state::set("MyApp", "Window", "Top", "40").unwrap();
            delete_setting(
                &VBString::from("MyApp"),
                &VBString::from("Window"),
                Some(&VBString::from("Left")),
            )
            .unwrap();
            assert_eq!(settings_state::get("MyApp", "Window", "Left"), None);
            assert_eq!(
                settings_state::get("MyApp", "Window", "Top").as_deref(),
                Some("40")
            );
        });
    }

    #[test]
    fn deletes_an_entire_section_when_key_is_empty() {
        with_temp_settings_store(|_| {
            settings_state::set("MyApp", "Window", "Left", "150").unwrap();
            settings_state::set("MyApp", "Window", "Top", "40").unwrap();
            delete_setting(&VBString::from("MyApp"), &VBString::from("Window"), None).unwrap();
            assert!(settings_state::get_all("MyApp", "Window").is_empty());
        });
    }

    #[test]
    fn deleting_nonexistent_setting_is_noop() {
        with_temp_settings_store(|_| {
            delete_setting(
                &VBString::from("MyApp"),
                &VBString::from("Missing"),
                Some(&VBString::from("Key")),
            )
            .unwrap();
            delete_setting(&VBString::from("MyApp"), &VBString::from("Missing"), None).unwrap();
        });
    }

    #[test]
    fn returns_empty_variant() {
        with_temp_settings_store(|_| {
            let result = delete_setting(
                &VBString::from("MyApp"),
                &VBString::from("Section"),
                Some(&VBString::from("Key")),
            )
            .unwrap();
            assert_eq!(result, VBVariant::Empty);
        });
    }
}
