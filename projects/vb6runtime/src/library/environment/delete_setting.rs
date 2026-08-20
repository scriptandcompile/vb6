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
use crate::value::VBVariant;

/// Deletes a section or key setting from the VB6 settings store.
///
/// When `key` is provided (not `Empty`), only that key is removed. When
/// `key` is `Empty` (omitted by the caller), the entire section is removed.
///
/// `Null` arguments raise error 94 (invalid use of `Null`); object and
/// array arguments raise error 13 (type mismatch).
pub fn delete_setting(
    appname: &VBVariant,
    section: &VBVariant,
    key: &VBVariant,
) -> VBResult<VBVariant> {
    let appname = appname.as_string()?;
    let section = section.as_string()?;
    if key.is_empty() {
        settings::remove_section(&appname, &section)
            .map_err(|e| crate::error::VBError::with_description(5, e.to_string()))?;
    } else {
        let key = key.as_string()?;
        settings::remove_key(&appname, &section, &key)
            .map_err(|e| crate::error::VBError::with_description(5, e.to_string()))?;
    }
    Ok(VBVariant::Empty)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::settings as settings_state;
    use crate::state::test_support::with_temp_settings_store;

    fn string(value: &str) -> VBVariant {
        VBVariant::from_string(value)
    }

    #[test]
    fn deletes_a_specific_key() {
        with_temp_settings_store(|_| {
            settings_state::set("MyApp", "Window", "Left", "150").unwrap();
            settings_state::set("MyApp", "Window", "Top", "40").unwrap();
            delete_setting(&string("MyApp"), &string("Window"), &string("Left")).unwrap();
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
            delete_setting(&string("MyApp"), &string("Window"), &VBVariant::Empty).unwrap();
            assert!(settings_state::get_all("MyApp", "Window").is_empty());
        });
    }

    #[test]
    fn deleting_nonexistent_setting_is_noop() {
        with_temp_settings_store(|_| {
            delete_setting(&string("MyApp"), &string("Missing"), &string("Key")).unwrap();
            delete_setting(&string("MyApp"), &string("Missing"), &VBVariant::Empty).unwrap();
        });
    }

    #[test]
    fn null_arguments_are_error_94() {
        with_temp_settings_store(|_| {
            let err =
                delete_setting(&VBVariant::Null, &string("Section"), &string("Key")).unwrap_err();
            assert_eq!(err.number, crate::error::err_number::INVALID_USE_OF_NULL);
            let err = delete_setting(&string("App"), &VBVariant::Null, &string("Key")).unwrap_err();
            assert_eq!(err.number, crate::error::err_number::INVALID_USE_OF_NULL);
            let err =
                delete_setting(&string("App"), &string("Section"), &VBVariant::Null).unwrap_err();
            assert_eq!(err.number, crate::error::err_number::INVALID_USE_OF_NULL);
        });
    }

    #[test]
    fn object_and_array_arguments_are_error_13() {
        with_temp_settings_store(|_| {
            let array = VBVariant::array_dynamic(vb6core::types::VBType::String);
            let err = delete_setting(&array, &string("Section"), &string("Key")).unwrap_err();
            assert_eq!(err.number, crate::error::err_number::TYPE_MISMATCH);
        });
    }

    #[test]
    fn returns_empty_variant() {
        with_temp_settings_store(|_| {
            let result =
                delete_setting(&string("MyApp"), &string("Section"), &string("Key")).unwrap();
            assert_eq!(result, VBVariant::Empty);
        });
    }
}
