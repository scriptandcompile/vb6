//! Win32 `.res` (compiled resource script) file parser.
//!
//! Backs the VB6 `LoadResData`, `LoadResPicture`, and `LoadResString` functions,
//! which read from the single `.res` file a VB6 project links at compile time.
//!
//! # File format
//!
//! A `.res` file is a flat, ordered sequence of *resource records* with no
//! global file header and no directory. Each record is self-describing and
//! carries its own header, so the file is parsed by walking record to record
//! until the buffer is exhausted. Every record has this layout:
//!
//! ```text
//! offset  size  field
//! ------  ----  ---------------------------------------------------------
//!      0     4  DataSize      byte length of the resource data
//!      4     4  HeaderSize    byte length of this header, data starts here
//!      8     *  Type          ordinal-or-name (see below)
//!      *     *  Name          ordinal-or-name (see below)
//!      *   0-2  padding       to align the following DWORD to 4 bytes
//!      *     4  DataVersion
//!      *     2  MemoryFlags
//!      *     2  LanguageId    Win32 LANGID, e.g. 0x0409 = en-US
//!      *     4  Version
//!      *     4  Characteristics
//!      *     *  data          DataSize bytes, then padded to a 4-byte bound
//! ```
//!
//! `Type` and `Name` are each an `ordinal-or-name`: if the first `u16` is
//! [`ORDINAL_MARKER`] (`0xFFFF`) the field is a 4-byte ordinal whose value is
//! the second `u16`; otherwise it is a NUL-terminated UTF-16LE string. This is
//! why the header is variable-length and why `HeaderSize` — not a fixed
//! constant — must be used to find the data.
//!
//! Note that `HeaderSize` is authoritative: it is used to locate the data, and
//! the fixed trailer is read at the position derived from the type/name fields.
//! Both are cross-checked, and a record whose `HeaderSize` disagrees with its
//! parsed contents is rejected as a malformed file.
//!
//! Every well-formed `.res` file begins with a *null record*: `Type` ordinal 0,
//! `Name` ordinal 0, and `DataSize` 0. It carries no resource and is skipped.
//!
//! # References
//!
//! - [RESOURCEHEADER structure](https://learn.microsoft.com/en-us/windows/win32/menurc/resourceheader)
//! - [Resource types](https://learn.microsoft.com/en-us/windows/win32/menurc/resource-types)

use crate::error::{err_number, VBError, VBResult};
use crate::state::file::{self, AccessMode, LockMode, OpenMode};

/// Sentinel `u16` marking an `ordinal-or-name` field as a numeric ordinal
/// rather than a UTF-16LE string.
const ORDINAL_MARKER: u16 = 0xFFFF;

/// Size in bytes of the `DataSize` + `HeaderSize` pair that opens every record.
const SIZE_PREFIX_LEN: usize = 8;

/// Size in bytes of the fixed trailer that closes every record header
/// (`DataVersion` + `MemoryFlags` + `LanguageId` + `Version` + `Characteristics`).
const HEADER_TRAILER_LEN: usize = 16;

/// Records, and the data within them, are aligned to a 4-byte boundary.
const RECORD_ALIGNMENT: usize = 4;

/// Round `value` up to the next [`RECORD_ALIGNMENT`] boundary.
fn align_up(value: usize) -> usize {
    value.next_multiple_of(RECORD_ALIGNMENT)
}

/// Standard Win32 resource type ordinals (`RT_*`), as used in the `Type` field.
///
/// VB6 exposes only a subset of these; the rest are recognized so unrelated
/// records (version info, manifests, etc.) can be walked over rather than
/// misinterpreted as VB6 resources.
pub mod rt {
    /// `RT_CURSOR`: single cursor image.
    pub const CURSOR: u16 = 1;
    /// `RT_BITMAP`: bitmap.
    pub const BITMAP: u16 = 2;
    /// `RT_ICON`: single icon image.
    pub const ICON: u16 = 3;
    /// `RT_MENU`: menu template.
    pub const MENU: u16 = 4;
    /// `RT_DIALOG`: dialog template.
    pub const DIALOG: u16 = 5;
    /// `RT_STRING`: string table bundle of 16 strings.
    pub const STRING: u16 = 6;
    /// `RT_FONTDIR`: font directory.
    pub const FONTDIR: u16 = 7;
    /// `RT_FONT`: font.
    pub const FONT: u16 = 8;
    /// `RT_ACCELERATOR`: accelerator table.
    pub const ACCELERATOR: u16 = 9;
    /// `RT_RCDATA`: application-defined raw data. VB6 custom resources use this.
    pub const RCDATA: u16 = 10;
    /// `RT_MESSAGETABLE`: message table.
    pub const MESSAGETABLE: u16 = 11;
    /// `RT_GROUP_CURSOR`: cursor directory naming `RT_CURSOR` members.
    pub const GROUP_CURSOR: u16 = 12;
    /// `RT_GROUP_ICON`: icon directory naming `RT_ICON` members.
    pub const GROUP_ICON: u16 = 14;
    /// `RT_VERSION`: version information.
    pub const VERSION: u16 = 16;
    /// `RT_MANIFEST`: side-by-side assembly manifest.
    pub const MANIFEST: u16 = 24;
}

/// VB6 `vbResBitmap`: the `LoadResPicture` format argument for a bitmap.
pub const VB_RES_BITMAP: i32 = 0;
/// VB6 `vbResIcon`: the `LoadResPicture` format argument for an icon.
pub const VB_RES_ICON: i32 = 1;
/// VB6 `vbResCursor`: the `LoadResPicture` format argument for a cursor.
pub const VB_RES_CURSOR: i32 = 2;

/// Number of strings bundled into a single `RT_STRING` resource.
///
/// Win32 stores string tables in blocks of 16. The resource holding string
/// `id` has bundle ordinal `id / 16 + 1`, and the string sits at index
/// `id % 16` within it.
pub const STRINGS_PER_BUNDLE: u16 = 16;

/// The `Type` or `Name` of a resource: either a numeric ordinal or a string.
///
/// VB6 resource IDs are normally ordinals, but the VB6 Resource Editor and
/// `rc.exe` both emit string names too (e.g. a custom `"DLL"` type).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResId {
    /// A numeric ordinal, e.g. `101` or an `RT_*` constant.
    Ordinal(u16),
    /// A string name, compared case-insensitively as Win32 does.
    Name(String),
}

impl ResId {
    /// The ordinal value, or `None` if this is a string name.
    pub fn as_ordinal(&self) -> Option<u16> {
        match self {
            Self::Ordinal(value) => Some(*value),
            Self::Name(_) => None,
        }
    }

    /// Whether this id matches `ordinal`.
    ///
    /// A string name that is a valid decimal number also matches that number:
    /// `rc.exe` emits types such as `"#24"` and some editors emit plain
    /// `"24"`, both of which denote the ordinal.
    pub fn matches_ordinal(&self, ordinal: u16) -> bool {
        match self {
            Self::Ordinal(value) => *value == ordinal,
            Self::Name(name) => name
                .strip_prefix('#')
                .unwrap_or(name)
                .parse::<u16>()
                .is_ok_and(|parsed| parsed == ordinal),
        }
    }

    /// Whether this id matches `name`, case-insensitively as Win32 compares.
    pub fn matches_name(&self, name: &str) -> bool {
        match self {
            Self::Name(value) => value.eq_ignore_ascii_case(name),
            Self::Ordinal(_) => false,
        }
    }
}

impl std::fmt::Display for ResId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ordinal(value) => write!(f, "{value}"),
            Self::Name(name) => write!(f, "{name}"),
        }
    }
}

/// A single resource record parsed from a `.res` file.
///
/// The record's data is not copied; `data_offset` and `data_size` locate it
/// within the owning [`ResFile`]'s buffer. Use [`ResFile::data`] to read it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResEntry {
    /// Resource type: an `RT_*` ordinal (see [`rt`]) or a custom string.
    pub res_type: ResId,
    /// Resource name: the ID passed to `LoadResData`/`LoadResPicture`.
    pub name: ResId,
    /// Win32 `LANGID` of this record, e.g. `0x0409` for en-US.
    ///
    /// The same name may appear once per language; VB6 picks by locale.
    pub language: u16,
    /// `MemoryFlags` bitfield from the header, preserved verbatim.
    pub memory_flags: u16,
    /// `DataVersion` field from the header, preserved verbatim.
    pub data_version: u32,
    /// `Version` field from the header, preserved verbatim.
    pub version: u32,
    /// `Characteristics` field from the header, preserved verbatim.
    pub characteristics: u32,
    /// Absolute byte offset of this record's data within the file buffer.
    pub data_offset: usize,
    /// Byte length of this record's data, excluding alignment padding.
    pub data_size: usize,
}

/// A parsed `.res` file and the buffer its entries reference.
#[derive(Debug, Clone)]
pub struct ResFile {
    /// Path the file was loaded from, as given by the caller.
    path: String,
    /// Complete file contents. Entries reference slices of this.
    buffer: Vec<u8>,
    /// Records in file order, with the leading null record dropped.
    entries: Vec<ResEntry>,
}

impl ResFile {
    /// Loads and parses the `.res` file at `path`.
    ///
    /// Reads through the runtime's configured file backend (see
    /// [`crate::state::file`]), so the same code works against the native
    /// filesystem and against the in-memory backend used for WASM and tests,
    /// and relative paths resolve against the runtime file root.
    ///
    /// # Errors
    ///
    /// - Error 67 (`Too many files`) if no file number is free.
    /// - Error 53 (`File not found`) if `path` does not exist, and other
    ///   file errors as mapped by [`VBError`]'s `io::Error` conversion.
    /// - Error 325 (`Invalid format in resource file`) if the contents are
    ///   not a well-formed `.res` file.
    pub fn load(path: &str) -> VBResult<Self> {
        let path_ref = std::path::Path::new(path);

        // Opening for Binary creates the file when it is absent, which is
        // correct for VB6's Open statement but wrong here: a missing resource
        // file is an error, not a new empty one. Check before opening.
        if !file::file_exists(path_ref) {
            return Err(VBError::new(err_number::FILE_NOT_FOUND));
        }

        let file_number = file::free_file(0);
        if file_number == 0 {
            return Err(VBError::new(err_number::TOO_MANY_FILES));
        }

        // Binary mode: .res files are arbitrary bytes with no line structure.
        file::open_file(
            path_ref,
            OpenMode::Binary,
            AccessMode::Read,
            LockMode::Shared,
            0,
            file_number,
        )?;

        // Read to the end even if it fails, so the file number is never leaked.
        let read = file::read_file_to_vec(file_number);
        let closed = file::close_file(file_number);
        let buffer = read?;
        closed?;

        Self::parse(path, buffer)
    }

    /// Parses an already-read `.res` file image.
    ///
    /// `path` is retained for diagnostics only and is not opened.
    ///
    /// # Errors
    ///
    /// Error 325 (`Invalid format in resource file`) if `buffer` is not a
    /// well-formed `.res` file: a truncated record, a header size that
    /// disagrees with the parsed type/name fields, a data size that runs past
    /// the end of the buffer, or a name field with no terminator.
    pub fn parse(path: &str, buffer: Vec<u8>) -> VBResult<Self> {
        let entries = parse_entries(&buffer)?;
        Ok(Self {
            path: path.to_string(),
            buffer,
            entries,
        })
    }

    /// The path this file was loaded from.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// The parsed records, in file order.
    pub fn entries(&self) -> &[ResEntry] {
        &self.entries
    }

    /// The number of parsed records, excluding the leading null record.
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// The raw data of `entry`.
    ///
    /// Infallible for any entry obtained from this `ResFile`: bounds are
    /// validated during parsing.
    ///
    /// # Panics
    ///
    /// Panics if `entry` came from a different `ResFile`, whose offsets do not
    /// apply to this buffer.
    pub fn data(&self, entry: &ResEntry) -> &[u8] {
        &self.buffer[entry.data_offset..entry.data_offset + entry.data_size]
    }

    /// Finds the entry with resource type `res_type` and the given ordinal `name`.
    ///
    /// When the same name exists in several languages, the first in file order
    /// is returned.
    pub fn find_by_ordinal(&self, res_type: u16, name: u16) -> Option<&ResEntry> {
        self.entries.iter().find(|entry| {
            entry.res_type.matches_ordinal(res_type) && entry.name.matches_ordinal(name)
        })
    }

    /// Finds the entry with resource type `res_type` and the given string `name`,
    /// compared case-insensitively as Win32 does.
    pub fn find_by_name(&self, res_type: u16, name: &str) -> Option<&ResEntry> {
        self.entries
            .iter()
            .find(|entry| entry.res_type.matches_ordinal(res_type) && entry.name.matches_name(name))
    }

    /// Finds the entry whose name is `name`, regardless of resource type.
    pub fn find_any_type(&self, name: &ResId) -> Option<&ResEntry> {
        self.entries.iter().find(|entry| match name {
            ResId::Ordinal(ordinal) => entry.name.matches_ordinal(*ordinal),
            ResId::Name(text) => entry.name.matches_name(text),
        })
    }

    /// All entries of resource type `res_type`, in file order.
    pub fn entries_of_type(&self, res_type: u16) -> impl Iterator<Item = &ResEntry> {
        self.entries
            .iter()
            .filter(move |entry| entry.res_type.matches_ordinal(res_type))
    }
}

/// Walks `buffer` record by record, returning every record but the leading
/// null record.
fn parse_entries(buffer: &[u8]) -> VBResult<Vec<ResEntry>> {
    let mut entries = Vec::new();
    let mut offset = 0usize;

    // A trailing run of fewer than SIZE_PREFIX_LEN bytes cannot start a
    // record; it is alignment padding at the end of the file.
    while offset + SIZE_PREFIX_LEN <= buffer.len() {
        let (entry, next_offset) = parse_record(buffer, offset)?;

        // The leading null record (type 0, name 0, no data) is a format
        // marker, not a resource, so it is not surfaced to callers.
        let is_null_record = entry.data_size == 0
            && entry.res_type == ResId::Ordinal(0)
            && entry.name == ResId::Ordinal(0);
        if !is_null_record {
            entries.push(entry);
        }

        // Guaranteed by parse_record, but assert it rather than risk spinning.
        debug_assert!(next_offset > offset, "record parse made no progress");
        offset = next_offset;
    }

    Ok(entries)
}

/// Parses the record starting at `offset`, returning it and the offset of the
/// next record.
fn parse_record(buffer: &[u8], offset: usize) -> VBResult<(ResEntry, usize)> {
    let data_size = read_u32(buffer, offset)? as usize;
    let header_size = read_u32(buffer, offset + 4)? as usize;

    // The header must at least hold the size prefix, two ordinal-or-name
    // fields, and the fixed trailer.
    let min_header_size = SIZE_PREFIX_LEN + 2 * ORDINAL_FIELD_LEN + HEADER_TRAILER_LEN;
    if header_size < min_header_size {
        return Err(invalid_format());
    }

    // Type and name are variable-length, so the trailer's position is derived
    // rather than fixed.
    let mut cursor = offset + SIZE_PREFIX_LEN;
    let res_type = read_res_id(buffer, &mut cursor)?;
    let name = read_res_id(buffer, &mut cursor)?;

    // The trailer is DWORD-aligned relative to the start of the record.
    cursor = offset + align_up(cursor - offset);

    let data_version = read_u32(buffer, cursor)?;
    let memory_flags = read_u16(buffer, cursor + 4)?;
    let language = read_u16(buffer, cursor + 6)?;
    let version = read_u32(buffer, cursor + 8)?;
    let characteristics = read_u32(buffer, cursor + 12)?;
    cursor += HEADER_TRAILER_LEN;

    // Cross-check the two independent descriptions of the header length. A
    // mismatch means the record is not laid out as the format requires.
    if cursor - offset != header_size {
        return Err(invalid_format());
    }

    let data_offset = offset + header_size;
    let data_end = data_offset
        .checked_add(data_size)
        .ok_or_else(invalid_format)?;
    if data_end > buffer.len() {
        return Err(invalid_format());
    }

    let next_offset = offset + align_up(header_size + data_size);

    Ok((
        ResEntry {
            res_type,
            name,
            language,
            memory_flags,
            data_version,
            version,
            characteristics,
            data_offset,
            data_size,
        },
        next_offset,
    ))
}

/// Byte length of an `ordinal-or-name` field in its ordinal form:
/// [`ORDINAL_MARKER`] followed by the ordinal value.
const ORDINAL_FIELD_LEN: usize = 4;

/// Reads an `ordinal-or-name` field at `*cursor`, advancing `cursor` past it.
fn read_res_id(buffer: &[u8], cursor: &mut usize) -> VBResult<ResId> {
    if read_u16(buffer, *cursor)? == ORDINAL_MARKER {
        let ordinal = read_u16(buffer, *cursor + 2)?;
        *cursor += ORDINAL_FIELD_LEN;
        return Ok(ResId::Ordinal(ordinal));
    }

    // NUL-terminated UTF-16LE. Unpaired surrogates are replaced rather than
    // rejected: a decodable name is more useful than failing the whole file.
    let mut units = Vec::new();
    loop {
        let unit = read_u16(buffer, *cursor)?;
        *cursor += 2;
        if unit == 0 {
            break;
        }
        units.push(unit);
    }

    Ok(ResId::Name(String::from_utf16_lossy(&units)))
}

/// Reads a little-endian `u16` at `offset`, or fails if it would run past the end.
fn read_u16(buffer: &[u8], offset: usize) -> VBResult<u16> {
    buffer
        .get(offset..offset + 2)
        .map(|bytes| u16::from_le_bytes([bytes[0], bytes[1]]))
        .ok_or_else(invalid_format)
}

/// Reads a little-endian `u32` at `offset`, or fails if it would run past the end.
fn read_u32(buffer: &[u8], offset: usize) -> VBResult<u32> {
    buffer
        .get(offset..offset + 4)
        .map(|bytes| u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
        .ok_or_else(invalid_format)
}

/// VB6 error 325: `Invalid format in resource file`.
fn invalid_format() -> VBError {
    VBError::new(err_number::INVALID_FORMAT_IN_RESOURCE_FILE)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds one `.res` record with an ordinal type and name.
    fn record(res_type: u16, name: u16, language: u16, data: &[u8]) -> Vec<u8> {
        let header_size = SIZE_PREFIX_LEN + 2 * ORDINAL_FIELD_LEN + HEADER_TRAILER_LEN;
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(data.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&(header_size as u32).to_le_bytes());
        bytes.extend_from_slice(&ORDINAL_MARKER.to_le_bytes());
        bytes.extend_from_slice(&res_type.to_le_bytes());
        bytes.extend_from_slice(&ORDINAL_MARKER.to_le_bytes());
        bytes.extend_from_slice(&name.to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes()); // DataVersion
        bytes.extend_from_slice(&0u16.to_le_bytes()); // MemoryFlags
        bytes.extend_from_slice(&language.to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes()); // Version
        bytes.extend_from_slice(&0u32.to_le_bytes()); // Characteristics
        bytes.extend_from_slice(data);
        bytes.resize(align_up(bytes.len()), 0);
        bytes
    }

    /// Builds one `.res` record with a string type and name.
    fn named_record(res_type: &str, name: &str, data: &[u8]) -> Vec<u8> {
        let mut fields = Vec::new();
        for text in [res_type, name] {
            for unit in text.encode_utf16() {
                fields.extend_from_slice(&unit.to_le_bytes());
            }
            fields.extend_from_slice(&0u16.to_le_bytes());
        }
        let header_size = align_up(SIZE_PREFIX_LEN + fields.len()) + HEADER_TRAILER_LEN;

        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(data.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&(header_size as u32).to_le_bytes());
        bytes.extend_from_slice(&fields);
        bytes.resize(align_up(bytes.len()), 0); // pad to the DWORD trailer
        bytes.extend_from_slice(&0u32.to_le_bytes()); // DataVersion
        bytes.extend_from_slice(&0u16.to_le_bytes()); // MemoryFlags
        bytes.extend_from_slice(&0x0409u16.to_le_bytes()); // LanguageId
        bytes.extend_from_slice(&0u32.to_le_bytes()); // Version
        bytes.extend_from_slice(&0u32.to_le_bytes()); // Characteristics
        bytes.extend_from_slice(data);
        bytes.resize(align_up(bytes.len()), 0);
        bytes
    }

    /// The null record every well-formed `.res` file starts with.
    fn null_record() -> Vec<u8> {
        record(0, 0, 0, &[])
    }

    #[test]
    fn null_record_is_not_surfaced_as_an_entry() {
        let file = ResFile::parse("t.res", null_record()).unwrap();
        assert_eq!(file.entry_count(), 0);
    }

    #[test]
    fn parses_ordinal_type_and_name_with_data() {
        let mut bytes = null_record();
        bytes.extend(record(rt::RCDATA, 101, 0x0409, b"payload"));

        let file = ResFile::parse("t.res", bytes).unwrap();
        assert_eq!(file.entry_count(), 1);

        let entry = &file.entries()[0];
        assert_eq!(entry.res_type, ResId::Ordinal(rt::RCDATA));
        assert_eq!(entry.name, ResId::Ordinal(101));
        assert_eq!(entry.language, 0x0409);
        assert_eq!(entry.data_size, 7);
        assert_eq!(file.data(entry), b"payload");
    }

    #[test]
    fn data_excludes_alignment_padding() {
        // 5 bytes of data pad out to 8; data() must return only the 5.
        let mut bytes = null_record();
        bytes.extend(record(rt::RCDATA, 1, 0, b"12345"));
        bytes.extend(record(rt::RCDATA, 2, 0, b"second"));

        let file = ResFile::parse("t.res", bytes).unwrap();
        assert_eq!(file.data(&file.entries()[0]), b"12345");
        assert_eq!(file.data(&file.entries()[1]), b"second");
    }

    #[test]
    fn walks_every_record_in_a_multi_record_file() {
        let mut bytes = null_record();
        for id in 1..=5u16 {
            bytes.extend(record(rt::ICON, id, 0x0409, &[id as u8; 3]));
        }

        let file = ResFile::parse("t.res", bytes).unwrap();
        assert_eq!(file.entry_count(), 5);
        for (index, entry) in file.entries().iter().enumerate() {
            let id = index as u16 + 1;
            assert_eq!(entry.name, ResId::Ordinal(id));
            assert_eq!(file.data(entry), &[id as u8; 3]);
        }
    }

    #[test]
    fn parses_string_type_and_name() {
        let mut bytes = null_record();
        bytes.extend(named_record("DLL", "FMOD", b"binary"));

        let file = ResFile::parse("t.res", bytes).unwrap();
        let entry = &file.entries()[0];
        assert_eq!(entry.res_type, ResId::Name("DLL".to_string()));
        assert_eq!(entry.name, ResId::Name("FMOD".to_string()));
        assert_eq!(file.data(entry), b"binary");
    }

    #[test]
    fn find_by_name_is_case_insensitive() {
        let mut bytes = null_record();
        bytes.extend(named_record("DLL", "FMOD", b"binary"));
        let file = ResFile::parse("t.res", bytes).unwrap();

        // Type "DLL" is a string, so look it up through the name-agnostic path.
        let entry = file
            .find_any_type(&ResId::Name("fmod".to_string()))
            .unwrap();
        assert_eq!(file.data(entry), b"binary");
        assert!(file
            .find_any_type(&ResId::Name("zlib".to_string()))
            .is_none());
    }

    #[test]
    fn numeric_string_type_matches_the_equivalent_ordinal() {
        // rc.exe writes types like "#24" for RT_MANIFEST.
        let mut bytes = null_record();
        bytes.extend(named_record("#24", "1", b"<assembly/>"));
        let file = ResFile::parse("t.res", bytes).unwrap();

        let entry = file.find_by_ordinal(rt::MANIFEST, 1).unwrap();
        assert_eq!(file.data(entry), b"<assembly/>");
    }

    #[test]
    fn find_by_ordinal_returns_the_matching_entry() {
        let mut bytes = null_record();
        bytes.extend(record(rt::ICON, 1, 0x0409, b"icon-one"));
        bytes.extend(record(rt::ICON, 2, 0x0409, b"icon-two"));
        bytes.extend(record(rt::RCDATA, 2, 0x0409, b"data-two"));

        let file = ResFile::parse("t.res", bytes).unwrap();
        assert_eq!(
            file.data(file.find_by_ordinal(rt::ICON, 2).unwrap()),
            b"icon-two"
        );
        assert_eq!(
            file.data(file.find_by_ordinal(rt::RCDATA, 2).unwrap()),
            b"data-two"
        );
        assert!(file.find_by_ordinal(rt::BITMAP, 2).is_none());
        assert!(file.find_by_ordinal(rt::ICON, 99).is_none());
    }

    #[test]
    fn entries_of_type_filters_by_resource_type() {
        let mut bytes = null_record();
        bytes.extend(record(rt::ICON, 1, 0, b"a"));
        bytes.extend(record(rt::RCDATA, 2, 0, b"b"));
        bytes.extend(record(rt::ICON, 3, 0, b"c"));

        let file = ResFile::parse("t.res", bytes).unwrap();
        let icon_names: Vec<_> = file
            .entries_of_type(rt::ICON)
            .map(|e| e.name.clone())
            .collect();
        assert_eq!(icon_names, vec![ResId::Ordinal(1), ResId::Ordinal(3)]);
    }

    #[test]
    fn same_name_in_two_languages_resolves_to_the_first() {
        let mut bytes = null_record();
        bytes.extend(record(rt::STRING, 1, 0x0409, b"english"));
        bytes.extend(record(rt::STRING, 1, 0x040C, b"french"));

        let file = ResFile::parse("t.res", bytes).unwrap();
        assert_eq!(file.entry_count(), 2);
        let entry = file.find_by_ordinal(rt::STRING, 1).unwrap();
        assert_eq!(entry.language, 0x0409);
        assert_eq!(file.data(entry), b"english");
    }

    #[test]
    fn zero_length_resource_data_is_valid() {
        let mut bytes = null_record();
        bytes.extend(record(rt::RCDATA, 7, 0, b""));

        let file = ResFile::parse("t.res", bytes).unwrap();
        let entry = file.find_by_ordinal(rt::RCDATA, 7).unwrap();
        assert_eq!(entry.data_size, 0);
        assert!(file.data(entry).is_empty());
    }

    #[test]
    fn empty_buffer_yields_no_entries() {
        let file = ResFile::parse("t.res", Vec::new()).unwrap();
        assert_eq!(file.entry_count(), 0);
    }

    #[test]
    fn truncated_header_is_rejected() {
        let mut bytes = null_record();
        bytes.extend(record(rt::RCDATA, 1, 0, b"payload"));
        bytes.truncate(bytes.len() - 12); // cut into the second record's data

        let error = ResFile::parse("t.res", bytes).unwrap_err();
        assert_eq!(error.number, err_number::INVALID_FORMAT_IN_RESOURCE_FILE);
    }

    #[test]
    fn data_size_past_end_of_buffer_is_rejected() {
        let mut bytes = null_record();
        bytes.extend(record(rt::RCDATA, 1, 0, b"payload"));
        // Claim far more data than the file holds.
        let data_size_at = null_record().len();
        bytes[data_size_at..data_size_at + 4].copy_from_slice(&0xFFFF_0000u32.to_le_bytes());

        let error = ResFile::parse("t.res", bytes).unwrap_err();
        assert_eq!(error.number, err_number::INVALID_FORMAT_IN_RESOURCE_FILE);
    }

    #[test]
    fn header_size_below_the_minimum_is_rejected() {
        let mut bytes = null_record();
        bytes[4..8].copy_from_slice(&8u32.to_le_bytes()); // no room for type/name/trailer

        let error = ResFile::parse("t.res", bytes).unwrap_err();
        assert_eq!(error.number, err_number::INVALID_FORMAT_IN_RESOURCE_FILE);
    }

    #[test]
    fn header_size_disagreeing_with_parsed_fields_is_rejected() {
        let mut bytes = null_record();
        // Ordinal type and name give a 32-byte header; claim 36.
        bytes[4..8].copy_from_slice(&36u32.to_le_bytes());
        bytes.resize(36, 0);

        let error = ResFile::parse("t.res", bytes).unwrap_err();
        assert_eq!(error.number, err_number::INVALID_FORMAT_IN_RESOURCE_FILE);
    }

    #[test]
    fn unterminated_string_name_is_rejected() {
        // A name field whose UTF-16 run never hits a NUL before end of buffer.
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&0u32.to_le_bytes()); // DataSize
        bytes.extend_from_slice(&32u32.to_le_bytes()); // HeaderSize
        bytes.extend_from_slice(&ORDINAL_MARKER.to_le_bytes());
        bytes.extend_from_slice(&rt::RCDATA.to_le_bytes());
        bytes.extend_from_slice(&[0x41, 0x00, 0x42, 0x00]); // "AB", no terminator

        let error = ResFile::parse("t.res", bytes).unwrap_err();
        assert_eq!(error.number, err_number::INVALID_FORMAT_IN_RESOURCE_FILE);
    }

    #[test]
    fn res_id_ordinal_and_name_accessors() {
        assert_eq!(ResId::Ordinal(42).as_ordinal(), Some(42));
        assert_eq!(ResId::Name("A".to_string()).as_ordinal(), None);
        assert!(ResId::Name("24".to_string()).matches_ordinal(24));
        assert!(ResId::Name("#24".to_string()).matches_ordinal(24));
        assert!(!ResId::Name("DLL".to_string()).matches_ordinal(24));
        assert!(!ResId::Ordinal(24).matches_name("24"));
        assert_eq!(ResId::Ordinal(7).to_string(), "7");
        assert_eq!(ResId::Name("LOGO".to_string()).to_string(), "LOGO");
    }

    // ---- loading through the runtime file backend ----

    /// Installs a fresh in-memory backend holding `content` at `name`, then
    /// runs `f`. Uses the memory backend so the test never touches the real
    /// filesystem, and serializes on the shared state lock.
    fn with_res_file<T>(name: &str, content: &[u8], f: impl FnOnce(&str) -> T) -> T {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();
        file::set_backend(Box::new(file::memory::MemoryBackend::new()));
        file::set_root("/");
        let path = format!("/{name}");
        file::write_memory_file(&path, content).unwrap();

        let result = f(&path);

        let _ = file::close_all_files();
        file::reset_backend();
        result
    }

    #[test]
    fn load_reads_through_the_file_backend() {
        let mut bytes = null_record();
        bytes.extend(record(rt::RCDATA, 101, 0x0409, b"from-backend"));

        with_res_file("app.res", &bytes, |path| {
            let file = ResFile::load(path).unwrap();
            assert_eq!(file.path(), path);
            assert_eq!(file.entry_count(), 1);
            let entry = file.find_by_ordinal(rt::RCDATA, 101).unwrap();
            assert_eq!(file.data(entry), b"from-backend");
        });
    }

    #[test]
    fn load_releases_the_file_number() {
        let bytes = null_record();
        with_res_file("app.res", &bytes, |path| {
            let before = file::free_file(0);
            ResFile::load(path).unwrap();
            assert_eq!(file::free_file(0), before, "file number leaked");
        });
    }

    #[test]
    fn load_reports_a_missing_file() {
        with_res_file("app.res", &null_record(), |_| {
            let error = ResFile::load("/absent.res").unwrap_err();
            assert_eq!(error.number, err_number::FILE_NOT_FOUND);
        });
    }

    #[test]
    fn load_reports_a_malformed_file() {
        // A header size that cannot hold the required fields.
        let bytes = vec![0u8; 8];
        with_res_file("bad.res", &bytes, |path| {
            let error = ResFile::load(path).unwrap_err();
            assert_eq!(error.number, err_number::INVALID_FORMAT_IN_RESOURCE_FILE);
        });
    }

    // ---- real .res files from the repository test data ----

    /// Loads a `.res` file from the repository `test-data` directory through
    /// the native backend rooted at the workspace root.
    fn load_test_data(relative_path: &str) -> ResFile {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();
        file::reset_backend();
        // vb6runtime/src/library/resources -> workspace root is 3 levels up.
        let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .expect("workspace root")
            .to_path_buf();
        file::set_root(workspace_root);

        let result = ResFile::load(relative_path);
        let _ = file::close_all_files();
        file::reset_backend();
        result.expect("test-data .res file should parse")
    }

    #[test]
    fn parses_xpmanifest_res_from_test_data() {
        let file =
            load_test_data("test-data/Bitrate-calculator/Windows/Source-code/XPManifest.res");

        // One RT_MANIFEST record holding the XML manifest.
        assert_eq!(file.entry_count(), 1);
        let entry = file.find_by_ordinal(rt::MANIFEST, 1).unwrap();
        assert_eq!(entry.data_size, 644);
        let xml = file.data(entry);
        assert!(xml.starts_with(b"<?xml version=\"1.0\""));
        assert!(xml.windows(8).any(|w| w == b"assembly"));
    }

    #[test]
    fn parses_mexe2_2_res_from_test_data() {
        let file = load_test_data("test-data/Environment/mexe2_2.res");

        // Three RT_ICON images, an RT_GROUP_ICON directory, and a manifest.
        assert_eq!(file.entry_count(), 5);

        let icons: Vec<_> = file.entries_of_type(rt::ICON).collect();
        assert_eq!(icons.len(), 3);
        assert_eq!(icons[0].data_size, 744);
        assert_eq!(icons[0].language, 0x0409);
        // Icon data is a BITMAPINFOHEADER, which opens with its own size (40).
        assert_eq!(&file.data(icons[0])[..4], &40u32.to_le_bytes());

        // The group icon record is named with the string "A" in this file.
        let group = file.find_any_type(&ResId::Name("A".to_string())).unwrap();
        assert_eq!(group.res_type, ResId::Ordinal(rt::GROUP_ICON));
        assert_eq!(group.data_size, 48);

        assert!(file.find_by_ordinal(rt::MANIFEST, 1).is_some());
    }

    #[test]
    fn parses_m2000_res_with_a_numeric_string_type() {
        // This file stores RT_MANIFEST as the string type "#24" with a
        // 36-byte header, exercising the variable-length header path.
        let file = load_test_data("test-data/Environment/M2000.RES");

        assert_eq!(file.entry_count(), 1);
        let entry = &file.entries()[0];
        assert_eq!(entry.res_type, ResId::Name("#24".to_string()));
        assert_eq!(entry.data_size, 1176);
        assert!(file.find_by_ordinal(rt::MANIFEST, 1).is_some());
    }

    #[test]
    fn parses_project1_res_with_string_types_and_names() {
        // Two large records with both type and name as strings.
        let file = load_test_data("test-data/CdiuBeatUpEditor/Project1.RES");

        assert_eq!(file.entry_count(), 2);
        let fmod = file
            .find_any_type(&ResId::Name("FMOD".to_string()))
            .unwrap();
        assert_eq!(fmod.res_type, ResId::Name("DLL".to_string()));
        assert_eq!(fmod.data_size, 161_280);
        assert_eq!(file.data(fmod).len(), 161_280);

        let zlib = file
            .find_any_type(&ResId::Name("ZLIB".to_string()))
            .unwrap();
        assert_eq!(zlib.data_size, 53_760);
    }
}
