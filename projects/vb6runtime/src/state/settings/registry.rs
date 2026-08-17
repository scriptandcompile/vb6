//! Windows Registry settings backend.
//!
//! Stores settings directly in the Windows registry under
//! `HKEY_CURRENT_USER\Software\VB and VBA Program Settings\appname\section\key`,
//! matching the native VB6 behavior.
//!
//! This backend is only available on Windows targets.

use std::collections::HashMap;
use std::io;

#[cfg(windows)]
use windows_sys::Win32::System::Registry::*;

use super::backend::SettingsBackend;
use super::{Entry, IndexKey};

/// The registry path prefix for VB6 settings.
#[cfg(windows)]
const BASE_KEY: &str = "Software\\VB and VBA Program Settings";

/// Build the full registry subkey path for `(appname, section)`.
#[cfg(windows)]
fn subkey_path(appname: &str, section: &str) -> String {
    format!("{BASE_KEY}\\{appname}\\{section}")
}

/// Windows Registry settings backend.
///
/// Uses `HKEY_CURRENT_USER\Software\VB and VBA Program Settings` as the
/// root, matching native VB6 `SaveSetting`/`GetSetting` behavior.
pub struct RegistryBackend;

impl RegistryBackend {
    /// Create a new registry backend.
    pub fn new() -> Self {
        Self
    }
}

impl Default for RegistryBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(windows)]
mod windows_impl {
    use super::*;
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    /// Convert a Rust string to a null-terminated UTF-16 vector for Windows APIs.
    fn to_wide(s: &str) -> Vec<u16> {
        OsStr::new(s)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect()
    }

    /// Open or create a registry key with the given access rights.
    ///
    /// Returns the handle on success, or an error on failure.
    fn open_key(subkey: &str, access: u32) -> io::Result<isize> {
        let subkey_wide = to_wide(subkey);
        let mut hkey = 0isize;

        let result = unsafe {
            RegCreateKeyExW(
                HKEY_CURRENT_USER,
                subkey_wide.as_ptr(),
                0,
                std::ptr::null_mut(),
                REG_OPTION_NON_VOLATILE,
                access,
                std::ptr::null_mut(),
                &mut hkey,
                std::ptr::null_mut(),
            )
        };

        if result != ERROR_SUCCESS {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to open registry key: error {result}"),
            ));
        }

        Ok(hkey)
    }

    impl SettingsBackend for RegistryBackend {
        fn get(&self, appname: &str, section: &str, key: &str) -> Option<String> {
            let subkey = super::subkey_path(appname, section);
            let subkey_wide = to_wide(&subkey);
            let key_name_wide = to_wide(key);

            unsafe {
                let mut hkey = 0isize;
                let result = RegOpenKeyExW(
                    HKEY_CURRENT_USER,
                    subkey_wide.as_ptr(),
                    0,
                    KEY_READ,
                    &mut hkey,
                );

                if result != ERROR_SUCCESS {
                    return None;
                }

                let mut value_type: u32 = 0;
                let mut value_size: u32 = 0;

                // Query size first
                RegQueryValueExW(
                    hkey,
                    key_name_wide.as_ptr(),
                    std::ptr::null_mut(),
                    &mut value_type,
                    std::ptr::null_mut(),
                    &mut value_size,
                );

                if value_size == 0 {
                    RegCloseKey(hkey);
                    return None;
                }

                let mut buffer = vec![0u16; (value_size / 2) as usize];
                let result = RegQueryValueExW(
                    hkey,
                    key_name_wide.as_ptr(),
                    std::ptr::null_mut(),
                    &mut value_type,
                    buffer.as_mut_ptr(),
                    &mut value_size,
                );

                RegCloseKey(hkey);

                if result != ERROR_SUCCESS {
                    return None;
                }

                String::from_utf16(&buffer)
                    .ok()
                    .map(|s| s.trim_end_matches('\0').to_string())
            }
        }

        fn set(&self, appname: &str, section: &str, key: &str, value: &str) -> io::Result<()> {
            let subkey = super::subkey_path(appname, section);
            let key_name_wide = to_wide(key);
            let value_wide = to_wide(value);

            let hkey = open_key(&subkey, KEY_WRITE)?;

            let result = unsafe {
                RegSetValueExW(
                    hkey,
                    key_name_wide.as_ptr(),
                    0,
                    REG_SZ,
                    value_wide.as_ptr(),
                    ((value_wide.len()) * std::mem::size_of::<u16>()) as u32,
                )
            };

            unsafe { RegCloseKey(hkey) };

            if result != ERROR_SUCCESS {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Failed to set registry value: error {result}"),
                ));
            }

            Ok(())
        }

        fn remove_key(&self, appname: &str, section: &str, key: &str) -> io::Result<()> {
            let subkey = super::subkey_path(appname, section);
            let subkey_wide = to_wide(&subkey);
            let key_name_wide = to_wide(key);

            unsafe {
                let mut hkey = 0isize;
                let result = RegOpenKeyExW(
                    HKEY_CURRENT_USER,
                    subkey_wide.as_ptr(),
                    0,
                    KEY_WRITE,
                    &mut hkey,
                );

                if result != ERROR_SUCCESS {
                    return Ok(()); // Key doesn't exist, nothing to remove
                }

                let result = RegDeleteValueW(hkey, key_name_wide.as_ptr());
                RegCloseKey(hkey);

                if result != ERROR_SUCCESS && result != ERROR_FILE_NOT_FOUND {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Failed to delete registry value: error {result}"),
                    ));
                }
            }

            Ok(())
        }

        fn remove_section(&self, appname: &str, section: &str) -> io::Result<()> {
            let subkey = super::subkey_path(appname, section);
            let subkey_wide = to_wide(&subkey);

            unsafe {
                let result = RegDeleteTreeW(HKEY_CURRENT_USER, subkey_wide.as_ptr());
                if result != ERROR_SUCCESS && result != ERROR_FILE_NOT_FOUND {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Failed to delete registry section: error {result}"),
                    ));
                }
            }

            Ok(())
        }

        fn remove_appname(&self, appname: &str) -> io::Result<()> {
            let subkey = format!("{}\\{}", super::BASE_KEY, appname);
            let subkey_wide = to_wide(&subkey);

            unsafe {
                let result = RegDeleteTreeW(HKEY_CURRENT_USER, subkey_wide.as_ptr());
                if result != ERROR_SUCCESS && result != ERROR_FILE_NOT_FOUND {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Failed to delete registry appname: error {result}"),
                    ));
                }
            }

            Ok(())
        }

        fn get_all(&self, appname: &str, section: &str) -> Vec<(String, String)> {
            let subkey = super::subkey_path(appname, section);
            let subkey_wide = to_wide(&subkey);
            let mut out = Vec::new();

            unsafe {
                let mut hkey = 0isize;
                let result = RegOpenKeyExW(
                    HKEY_CURRENT_USER,
                    subkey_wide.as_ptr(),
                    0,
                    KEY_READ,
                    &mut hkey,
                );

                if result != ERROR_SUCCESS {
                    return out;
                }

                let mut index = 0;
                loop {
                    let mut name_buffer = [0u16; 256];
                    let mut name_size = name_buffer.len() as u32;

                    let result = RegEnumValueW(
                        hkey,
                        index,
                        name_buffer.as_mut_ptr(),
                        &mut name_size,
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                    );

                    if result != ERROR_SUCCESS {
                        break;
                    }

                    let name = String::from_utf16(&name_buffer[..name_size as usize])
                        .unwrap_or_default();

                    // Now query the value
                    let mut value_type: u32 = 0;
                    let mut value_size: u32 = 0;

                    let name_wide = to_wide(&name);
                    RegQueryValueExW(
                        hkey,
                        name_wide.as_ptr(),
                        std::ptr::null_mut(),
                        &mut value_type,
                        std::ptr::null_mut(),
                        &mut value_size,
                    );

                    if value_size > 0 {
                        let mut buffer = vec![0u16; (value_size / 2) as usize];
                        let result = RegQueryValueExW(
                            hkey,
                            name_wide.as_ptr(),
                            std::ptr::null_mut(),
                            &mut value_type,
                            buffer.as_mut_ptr(),
                            &mut value_size,
                        );

                        if result == ERROR_SUCCESS {
                            let value = String::from_utf16(&buffer)
                                .unwrap_or_default()
                                .trim_end_matches('\0')
                                .to_string();
                            out.push((name, value));
                        }
                    }

                    index += 1;
                }

                RegCloseKey(hkey);
            }

            out.sort();
            out
        }

        fn entries(&self) -> Vec<(String, String, String, String)> {
            let mut out = Vec::new();
            let base_wide = to_wide(super::BASE_KEY);

            unsafe {
                let mut hkey = 0isize;
                let result = RegOpenKeyExW(
                    HKEY_CURRENT_USER,
                    base_wide.as_ptr(),
                    0,
                    KEY_READ,
                    &mut hkey,
                );

                if result != ERROR_SUCCESS {
                    return out;
                }

                // Enumerate appnames
                let mut app_index = 0;
                loop {
                    let mut appname_buffer = [0u16; 256];
                    let mut appname_size = appname_buffer.len() as u32;

                    let result = RegEnumKeyExW(
                        hkey,
                        app_index,
                        appname_buffer.as_mut_ptr(),
                        &mut appname_size,
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                    );

                    if result != ERROR_SUCCESS {
                        break;
                    }

                    let appname = String::from_utf16(&appname_buffer[..appname_size as usize])
                        .unwrap_or_default();

                    // Open appname key to enumerate sections
                    let appname_wide = to_wide(&appname);
                    let mut app_hkey = 0isize;
                    let result = RegOpenKeyExW(hkey, appname_wide.as_ptr(), 0, KEY_READ, &mut app_hkey);

                    if result == ERROR_SUCCESS {
                        let mut sec_index = 0;
                        loop {
                            let mut section_buffer = [0u16; 256];
                            let mut section_size = section_buffer.len() as u32;

                            let result = RegEnumKeyExW(
                                app_hkey,
                                sec_index,
                                section_buffer.as_mut_ptr(),
                                &mut section_size,
                                std::ptr::null_mut(),
                                std::ptr::null_mut(),
                                std::ptr::null_mut(),
                                std::ptr::null_mut(),
                            );

                            if result != ERROR_SUCCESS {
                                break;
                            }

                            let section =
                                String::from_utf16(&section_buffer[..section_size as usize])
                                    .unwrap_or_default();

                            // Get all values for this section
                            let settings =
                                <Self as SettingsBackend>::get_all(self, &appname, &section);
                            for (key, value) in settings {
                                out.push((
                                    appname.clone(),
                                    section.clone(),
                                    key,
                                    value,
                                ));
                            }

                            sec_index += 1;
                        }

                        RegCloseKey(app_hkey);
                    }

                    app_index += 1;
                }

                RegCloseKey(hkey);
            }

            out.sort();
            out
        }

        fn load_all(&self) -> HashMap<IndexKey, Entry> {
            let mut state = HashMap::new();

            for (appname, section, key, value) in self.entries() {
                let index = super::index_key(&appname, &section, &key);
                state.insert(
                    index,
                    Entry {
                        path: PathCase {
                            appname,
                            section,
                            key,
                        },
                        value,
                    },
                );
            }

            state
        }
    }
}

#[cfg(not(windows))]
impl SettingsBackend for RegistryBackend {
    fn get(&self, _appname: &str, _section: &str, _key: &str) -> Option<String> {
        None
    }

    fn set(&self, _appname: &str, _section: &str, _key: &str, _value: &str) -> io::Result<()> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Registry backend is only available on Windows",
        ))
    }

    fn remove_key(&self, _appname: &str, _section: &str, _key: &str) -> io::Result<()> {
        Ok(())
    }

    fn remove_section(&self, _appname: &str, _section: &str) -> io::Result<()> {
        Ok(())
    }

    fn remove_appname(&self, _appname: &str) -> io::Result<()> {
        Ok(())
    }

    fn get_all(&self, _appname: &str, _section: &str) -> Vec<(String, String)> {
        Vec::new()
    }

    fn entries(&self) -> Vec<(String, String, String, String)> {
        Vec::new()
    }

    fn load_all(&self) -> HashMap<IndexKey, Entry> {
        HashMap::new()
    }
}

#[cfg(test)]
mod tests {
    // Registry tests are only meaningful on Windows
    #[cfg(windows)]
    mod windows_tests {
        use super::super::*;

        #[test]
        fn set_then_get_roundtrips() {
            let backend = RegistryBackend::new();
            // Use a unique appname to avoid test interference
            let appname = format!("VB6RuntimeTest_{}", std::process::id());
            backend.set(&appname, "TestSection", "TestKey", "TestValue").unwrap();
            assert_eq!(
                backend.get(&appname, "TestSection", "TestKey").as_deref(),
                Some("TestValue")
            );
            // Cleanup
            let _ = backend.remove_appname(&appname);
        }

        #[test]
        fn lookup_is_case_insensitive() {
            let backend = RegistryBackend::new();
            let appname = format!("VB6RuntimeTest_{}", std::process::id());
            backend.set(&appname, "Window", "Width", "600").unwrap();
            assert_eq!(
                backend.get(&appname, "window", "width").as_deref(),
                Some("600")
            );
            assert_eq!(
                backend.get(&appname, "WINDOW", "WIDTH").as_deref(),
                Some("600")
            );
            // Cleanup
            let _ = backend.remove_appname(&appname);
        }
    }
}
