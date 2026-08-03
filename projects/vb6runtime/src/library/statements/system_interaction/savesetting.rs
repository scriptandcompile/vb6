//! # `SaveSetting` Statement
//!
//! Saves or creates an application entry in the Windows registry or (on the Macintosh) information in the application's initialization file.
//!
//! ## Syntax
//!
//! ```vb
//! SaveSetting appname, section, key, setting
//! ```
//!
//! ## Parts
//!
//! - **appname**: Required. String expression containing the name of the application or project to which the setting applies.
//! - **section**: Required. String expression containing the name of the section in which the key setting is being saved.
//! - **key**: Required. String expression containing the name of the key setting being saved.
//! - **setting**: Required. Expression containing the value to which key is being set.
//!
//! ## Remarks
//!
//! - **Registry Location**: On Windows, `SaveSetting` writes to the registry under the path:
//!   `HKEY_CURRENT_USER\Software\VB and VBA Program Settings\appname\section\key`
//! - **String Values**: The setting argument is always stored as a string value in the registry.
//! - **Creating Entries**: If the specified key setting doesn't exist, `SaveSetting` creates it.
//! - **Creating Sections**: If the specified section doesn't exist, `SaveSetting` creates it.
//! - **Application Name**: The appname is typically the name of your application. Multiple applications can use the same registry location by using the same appname.
//! - **Section Organization**: Use sections to organize related settings. For example, you might have a "Startup" section and a "Display" section.
//! - **Type Conversion**: Numeric values and other data types are automatically converted to strings when saved.
//! - **Security**: Settings are stored per user (`HKEY_CURRENT_USER`), not per machine.
//! - **`GetSetting` Function**: Use the `GetSetting` function to retrieve values saved with `SaveSetting`.
//! - **`DeleteSetting` Statement**: Use `DeleteSetting` to remove registry entries created by `SaveSetting`.
//!
//! ## Examples
//!
//! ### Save a Simple Setting
//!
//! ```vb
//! SaveSetting "MyApp", "Startup", "Left", 100
//! SaveSetting "MyApp", "Startup", "Top", 100
//! ```
//!
//! ### Save User Preferences
//!
//! ```vb
//! SaveSetting "MyApp", "Preferences", "BackColor", vbBlue
//! SaveSetting "MyApp", "Preferences", "FontName", "Arial"
//! SaveSetting "MyApp", "Preferences", "FontSize", 12
//! ```
//!
//! ### Save Form Position on Close
//!
//! ```vb
//! Private Sub Form_Unload(Cancel As Integer)
//!     SaveSetting App.Title, "Position", "Left", Me.Left
//!     SaveSetting App.Title, "Position", "Top", Me.Top
//!     SaveSetting App.Title, "Position", "Width", Me.Width
//!     SaveSetting App.Title, "Position", "Height", Me.Height
//! End Sub
//! ```
//!
//! ### Save Boolean Settings
//!
//! ```vb
//! ' Save a boolean as a string
//! SaveSetting "MyApp", "Options", "AutoSave", CStr(chkAutoSave.Value)
//! ```
//!
//! ### Save with Variables
//!
//! ```vb
//! Dim userName As String
//! userName = txtUserName.Text
//! SaveSetting "MyApp", "User", "LastUser", userName
//! ```
//!
//! ### Save Multiple Related Settings
//!
//! ```vb
//! Sub SaveWindowSettings()
//!     Dim appName As String
//!     appName = App.Title
//!     
//!     SaveSetting appName, "Window", "Maximized", Me.WindowState = vbMaximized
//!     SaveSetting appName, "Window", "Visible", Me.Visible
//!     SaveSetting appName, "Window", "Caption", Me.Caption
//! End Sub
//! ```
//!
//! ## Common Patterns
//!
//! ### Using App.Title for Application Name
//!
//! ```vb
//! ' Ensures consistent application name across all settings
//! SaveSetting App.Title, "Database", "ConnectionString", connStr
//! ```
//!
//! ### Organizing Settings by Feature
//!
//! ```vb
//! ' Group related settings in sections
//! SaveSetting "MyApp", "Display", "Theme", "Dark"
//! SaveSetting "MyApp", "Display", "Language", "English"
//! SaveSetting "MyApp", "Network", "Port", 8080
//! SaveSetting "MyApp", "Network", "Timeout", 30
//! ```
//!
//! ## Important Notes
//!
//! - **Platform Differences**: On Windows, settings are stored in the registry. On other platforms, behavior may vary.
//! - **String Storage**: All values are stored as strings, so you may need to convert them back when retrieving with `GetSetting`.
//! - **Registry Cleanup**: Use `DeleteSetting` to remove settings when they're no longer needed.
//! - **Error Handling**: `SaveSetting` can fail if the registry is locked or permissions are insufficient.
//!
//! ## See Also
//!
//! - `GetSetting` function (retrieve saved settings)
//! - `GetAllSettings` function (retrieve all settings from a section)
//! - `DeleteSetting` statement (delete registry entries)
//!
//! ## References
//!
//! - [SaveSetting Statement - Microsoft Docs](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/savesetting-statement)
