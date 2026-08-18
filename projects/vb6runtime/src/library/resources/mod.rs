//! VB6 resources functions.
//!
//! `LoadResData`, `LoadResPicture`, and `LoadResString` all read from the one
//! `.res` file a VB6 project links at compile time, bound at runtime through
//! [`crate::state::resources`]. The `.res` parser itself lives in [`resfile`].

pub mod loadresdata;
pub mod loadrespicture;
pub mod loadresstring;
pub mod resfile;

use crate::error::{err_number, VBError, VBResult};
use crate::value::VBVariant;
use resfile::ResId;

/// Converts a `LoadRes*` index argument into a resource id.
///
/// VB6 accepts either a numeric ID or a string name for `LoadResData` and
/// `LoadResPicture`. Numeric values are taken as ordinals; anything else is
/// converted to a string and used as a name.
///
/// # Errors
///
/// Error 326 (`Resource with identifier not found`) if the value is `Null`,
/// `Empty`, an empty string, or a number outside the `u16` ordinal range —
/// none of which can name a resource, so no resource can be found for them.
/// Ordinal 0 is accepted: it is a valid string-table ID, and for the other
/// resource types the lookup simply finds nothing and reports 326 anyway.
pub(crate) fn index_to_res_id(index: &VBVariant) -> VBResult<ResId> {
    if index.is_null() || index.is_empty() {
        return Err(resource_not_found());
    }

    if index.is_numeric() {
        let ordinal = index.as_i32().map_err(|_| resource_not_found())?;
        let ordinal = u16::try_from(ordinal).map_err(|_| resource_not_found())?;
        return Ok(ResId::Ordinal(ordinal));
    }

    // A string index may still be a decimal ID written as text; ResId::Name
    // matching handles that, so keep it as a name here.
    let name = index.as_string().map_err(|_| resource_not_found())?;
    if name.is_empty() {
        return Err(resource_not_found());
    }
    Ok(ResId::Name(name))
}

/// VB6 error 326: `Resource with identifier not found`.
///
/// Raised by every `LoadRes*` function when the requested resource is absent,
/// and when no resource file is linked at all.
pub(crate) fn resource_not_found() -> VBError {
    VBError::new(err_number::RESOURCE_NOT_FOUND)
}

#[cfg(test)]
pub(crate) mod test_support {
    //! Shared fixtures for the `LoadRes*` tests.
    //!
    //! Builds `.res` images in memory and links them through the memory file
    //! backend, so no test touches the real filesystem.

    use crate::state::{file, resources};

    /// Byte length of a record header whose type and name are both ordinals.
    const ORDINAL_HEADER_SIZE: u32 = 32;

    /// Builds a `.res` record with an ordinal type and name, language en-US.
    pub(crate) fn record(res_type: u16, name: u16, data: &[u8]) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(data.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&ORDINAL_HEADER_SIZE.to_le_bytes());
        bytes.extend_from_slice(&0xFFFFu16.to_le_bytes());
        bytes.extend_from_slice(&res_type.to_le_bytes());
        bytes.extend_from_slice(&0xFFFFu16.to_le_bytes());
        bytes.extend_from_slice(&name.to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes()); // DataVersion
        bytes.extend_from_slice(&0u16.to_le_bytes()); // MemoryFlags
        bytes.extend_from_slice(&0x0409u16.to_le_bytes()); // LanguageId (en-US)
        bytes.extend_from_slice(&0u32.to_le_bytes()); // Version
        bytes.extend_from_slice(&0u32.to_le_bytes()); // Characteristics
        bytes.extend_from_slice(data);
        bytes.resize(bytes.len().next_multiple_of(4), 0);
        bytes
    }

    /// Builds a `.res` record with a string type and name.
    pub(crate) fn named_record(res_type: &str, name: &str, data: &[u8]) -> Vec<u8> {
        let mut fields = Vec::new();
        for text in [res_type, name] {
            for unit in text.encode_utf16() {
                fields.extend_from_slice(&unit.to_le_bytes());
            }
            fields.extend_from_slice(&0u16.to_le_bytes());
        }
        let header_size = (8 + fields.len()).next_multiple_of(4) + 16;

        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(data.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&(header_size as u32).to_le_bytes());
        bytes.extend_from_slice(&fields);
        bytes.resize(bytes.len().next_multiple_of(4), 0);
        bytes.extend_from_slice(&0u32.to_le_bytes()); // DataVersion
        bytes.extend_from_slice(&0u16.to_le_bytes()); // MemoryFlags
        bytes.extend_from_slice(&0x0409u16.to_le_bytes()); // LanguageId
        bytes.extend_from_slice(&0u32.to_le_bytes()); // Version
        bytes.extend_from_slice(&0u32.to_le_bytes()); // Characteristics
        bytes.extend_from_slice(data);
        bytes.resize(bytes.len().next_multiple_of(4), 0);
        bytes
    }

    /// The null record every well-formed `.res` file begins with.
    pub(crate) fn null_record() -> Vec<u8> {
        record(0, 0, &[])
    }

    /// Links a real `.res` file from the repository `test-data` directory and
    /// runs `f`, using the native backend rooted at the workspace root.
    ///
    /// Exercises the `LoadRes*` functions against genuine VB6-authored
    /// resource files rather than only synthesized ones.
    pub(crate) fn with_test_data_res<T>(relative_path: &str, f: impl FnOnce() -> T) -> T {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();
        file::reset_backend();
        // src/library/resources -> workspace root is 3 levels above the crate.
        let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|path| path.parent())
            .expect("workspace root")
            .to_path_buf();
        file::set_root(workspace_root);
        resources::clear();
        resources::set_file(relative_path);

        let result = f();

        resources::clear();
        let _ = file::close_all_files();
        file::reset_backend();
        result
    }

    /// Links `image` as the project's `.res` file and runs `f`.
    ///
    /// Serializes on the shared state lock and restores the default backend
    /// and unlinked resource state afterwards.
    pub(crate) fn with_linked_res<T>(image: &[u8], f: impl FnOnce() -> T) -> T {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();
        file::set_backend(Box::new(file::memory::MemoryBackend::new()));
        file::set_root("/");
        file::write_memory_file("/app.res", image).unwrap();
        resources::clear();
        resources::set_file("/app.res");

        let result = f();

        resources::clear();
        let _ = file::close_all_files();
        file::reset_backend();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_index_becomes_an_ordinal() {
        assert_eq!(
            index_to_res_id(&VBVariant::from_integer(101)).unwrap(),
            ResId::Ordinal(101)
        );
        assert_eq!(
            index_to_res_id(&VBVariant::from_long(65535)).unwrap(),
            ResId::Ordinal(65535)
        );
    }

    #[test]
    fn string_index_becomes_a_name() {
        assert_eq!(
            index_to_res_id(&VBVariant::from_string("LOGO")).unwrap(),
            ResId::Name("LOGO".to_string())
        );
    }

    #[test]
    fn ordinal_zero_is_a_usable_string_table_id() {
        assert_eq!(
            index_to_res_id(&VBVariant::from_long(0)).unwrap(),
            ResId::Ordinal(0)
        );
    }

    #[test]
    fn unusable_indexes_report_resource_not_found() {
        for index in [
            VBVariant::Null,
            VBVariant::Empty,
            VBVariant::from_string(""),
            VBVariant::from_long(-1),
            VBVariant::from_long(70000),
        ] {
            let error = index_to_res_id(&index).unwrap_err();
            assert_eq!(
                error.number,
                err_number::RESOURCE_NOT_FOUND,
                "index {index:?} should be rejected"
            );
        }
    }
}
