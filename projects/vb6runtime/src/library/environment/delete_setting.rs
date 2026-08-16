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
