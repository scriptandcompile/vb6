//! # `SavePicture` Statement
//!
//! Saves a graphical image from a control or form to a file.
//!
//! ## Syntax
//!
//! ```vb
//! SavePicture picture, stringexpression
//! ```
//!
//! ## Parts
//!
//! - **picture**: Required. A property or graphic object from which to save the image. The image
//!   can be from the `Picture` property of a Form, `PictureBox`, or Image control, or from the
//!   `Image` property of a `PictureBox` or Form.
//! - **stringexpression**: Required. String expression specifying the name of the file to which
//!   the graphic is saved. Can include a drive and path specification.
//!
//! ## Remarks
//!
//! - **File Format**: `SavePicture` saves graphics in bitmap (.bmp) format. The file created is
//!   compatible with bitmap files created by other applications.
//! - **Picture Property**: When used with the `Picture` property, `SavePicture` saves the persistent
//!   bitmap from the property. This is the image loaded at design time or assigned at run time via
//!   `LoadPicture` or other means.
//! - **Image Property**: When used with the `Image` property, `SavePicture` saves the current
//!   appearance of the form or picture box, including any graphics drawn with graphics methods.
//!   This creates a snapshot of the visible content.
//! - **File Path**: If no path is specified, the file is saved in the current directory.
//! - **Overwriting**: If a file with the specified name already exists, it is overwritten without
//!   warning.
//! - **Relative Paths**: You can use relative path specifications (e.g., "..\Images\MyPic.bmp").
//! - **Graphics Methods**: To save graphics created with graphics methods (Line, Circle, `PSet`,
//!   etc.), you must use the `Image` property, not the `Picture` property.
//! - **Clipboard Graphics**: `SavePicture` can also be used with graphics from the Clipboard object.
//!
//! ## Examples
//!
//! ### Save Form's Picture Property
//!
//! ```vb
//! ' Save the persistent bitmap from a form
//! SavePicture Form1.Picture, "C:\Images\Form1.bmp"
//! ```
//!
//! ### Save Form's Current Appearance
//!
//! ```vb
//! ' Save the current appearance of a form (including drawn graphics)
//! SavePicture Form1.Image, "C:\Images\FormSnapshot.bmp"
//! ```
//!
//! ### Save `PictureBox` Image
//!
//! ```vb
//! ' Save the picture from a PictureBox control
//! SavePicture Picture1.Picture, "C:\Temp\MyPicture.bmp"
//! ```
//!
//! ### Save with Variable Path
//!
//! ```vb
//! Dim FileName As String
//! FileName = "C:\Output\Image_" & Format$(Now, "yyyymmdd_hhnnss") & ".bmp"
//! SavePicture Picture1.Image, FileName
//! ```
//!
//! ### Save Clipboard Image
//!
//! ```vb
//! ' Save an image from the clipboard
//! SavePicture Clipboard.GetData(), "C:\Temp\ClipImage.bmp"
//! ```
//!
//! ### Error Handling
//!
//! ```vb
//! On Error Resume Next
//! SavePicture Picture1.Picture, "C:\Images\Output.bmp"
//! If Err.Number <> 0 Then
//!     MsgBox "Error saving picture: " & Err.Description
//! End If
//! On Error GoTo 0
//! ```
//!
//! ## Common Errors
//!
//! - **Error 53**: File not found - the specified path does not exist
//! - **Error 75**: Path/File access error - insufficient permissions or read-only file
//! - **Error 76**: Path not found - invalid directory path
//!
//! ## See Also
//!
//! - `LoadPicture` function (load images from files)
//! - `Picture` property (persistent bitmap property)
//! - `Image` property (current appearance snapshot)
//! - Graphics methods (`Line`, `Circle`, `PSet`, etc.)
//!
//! ## References
//!
//! - [SavePicture Statement - Microsoft Docs](https://learn.microsoft.com/en-us/previous-versions/visualstudio/visual-basic-6/aa268097(v=vs.60))

use crate::error::{VBError, VBResult};
use crate::state::file::{self, AccessMode, LockMode, OpenMode};
use crate::value::{VBString, VBVariant};
use crate::StdPicture;
use std::path::Path;
use vb6core::error::err_number;

/// Implementation of the `SavePicture` statement.
///
/// VB6 behavior:
/// - Saves the picture as a bitmap (.bmp) file, overwriting any existing file
///   without warning
/// - Accepts a `StdPicture` object (from `LoadPicture` or a `Picture`/`Image`
///   property)
/// - Saving `Nothing` raises error 91 (object variable not set)
/// - A non-picture value raises error 13 (type mismatch)
/// - An empty filename raises error 75 (path/file access error)
/// - Missing parent directories are created, matching the file backend's
///   `Output` mode behavior
pub fn save_picture(picture: &VBVariant, filename: &VBVariant) -> VBResult<()> {
    let object = match picture {
        VBVariant::Object(object) => object,
        VBVariant::Nothing => {
            return Err(VBError::new(err_number::OBJECT_VARIABLE_NOT_SET));
        }
        _ => return Err(VBError::type_mismatch()),
    };
    let picture = object
        .as_any()
        .downcast_ref::<StdPicture>()
        .ok_or_else(VBError::type_mismatch)?;

    // `VBString::try_from` raises error 94 for a Null filename.
    let path = VBString::try_from(filename)?;
    if path.as_str().is_empty() {
        return Err(VBError::new(err_number::PATH_FILE_ACCESS_ERROR));
    }

    let bytes = bitmap_bytes(picture.width(), picture.height());

    // Write through the file backend so native and WASM runs behave alike.
    // The internal file number is taken from the free pool and released
    // before returning.
    let number = file::free_file(file::MIN_FILE_NUMBER);
    if number == 0 {
        return Err(VBError::new(err_number::TOO_MANY_FILES));
    }

    let opened = file::open_file(
        Path::new(path.as_str()),
        OpenMode::Output,
        AccessMode::Write,
        LockMode::Shared,
        0,
        number,
    );
    if let Err(error) = opened {
        return Err(map_io_error(error));
    }

    let written = file::write_file(number, &bytes);
    // Close regardless of the write outcome so the handle is not leaked.
    let closed = file::close_file(number);
    written.map_err(map_io_error)?;
    closed.map_err(map_io_error)?;
    Ok(())
}

/// Map a backend I/O failure onto VB6 runtime errors.
fn map_io_error(error: std::io::Error) -> VBError {
    let number = match error.kind() {
        // Creating the output file failed because a path component is missing.
        std::io::ErrorKind::NotFound => err_number::PATH_NOT_FOUND,
        _ => err_number::PATH_FILE_ACCESS_ERROR,
    };
    VBError::with_description(number, error.to_string())
}

/// Build a 24-bit uncompressed BMP file body for a picture of the given size.
///
/// The runtime's [`StdPicture`] carries dimensions but not pixel data (see
/// `loadpicture`), so the bitmap is generated with a white background, like
/// the undrawn surface of a form or picture box.
fn bitmap_bytes(width: i32, height: i32) -> Vec<u8> {
    let width = width.max(1) as usize;
    let height = height.max(1) as usize;
    // Rows are stored bottom-up and padded to 4-byte boundaries.
    let row_size = (width * 3).div_ceil(4) * 4;
    let pixel_size = row_size * height;
    let data_offset: u32 = 54; // BITMAPFILEHEADER (14) + BITMAPINFOHEADER (40)
    let file_size = data_offset as usize + pixel_size;

    let mut bytes = Vec::with_capacity(file_size);

    // BITMAPFILEHEADER
    bytes.extend_from_slice(b"BM");
    bytes.extend_from_slice(&(file_size as u32).to_le_bytes());
    bytes.extend_from_slice(&0u32.to_le_bytes()); // reserved
    bytes.extend_from_slice(&data_offset.to_le_bytes());

    // BITMAPINFOHEADER
    bytes.extend_from_slice(&40u32.to_le_bytes()); // header size
    bytes.extend_from_slice(&(width as u32).to_le_bytes());
    bytes.extend_from_slice(&(height as u32).to_le_bytes());
    bytes.extend_from_slice(&1u16.to_le_bytes()); // color planes
    bytes.extend_from_slice(&24u16.to_le_bytes()); // bits per pixel
    bytes.extend_from_slice(&0u32.to_le_bytes()); // BI_RGB, no compression
    bytes.extend_from_slice(&(pixel_size as u32).to_le_bytes());
    bytes.extend_from_slice(&2835u32.to_le_bytes()); // 72 DPI
    bytes.extend_from_slice(&2835u32.to_le_bytes()); // 72 DPI
    bytes.extend_from_slice(&0u32.to_le_bytes()); // colors used
    bytes.extend_from_slice(&0u32.to_le_bytes()); // important colors

    // Pixel data: bottom-up BGR rows on a white background.
    for _ in 0..height {
        for _ in 0..width {
            bytes.extend_from_slice(&[0xFF, 0xFF, 0xFF]); // B, G, R
        }
        bytes.resize(bytes.len() + row_size - width * 3, 0);
    }

    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    fn picture_variant(width: i32, height: i32) -> VBVariant {
        VBVariant::from_object(Box::new(StdPicture::new(width, height)))
    }

    /// Installs a fresh temporary directory as the relative-path root.
    ///
    /// Serializes against the shared file-state snapshot: parallel runs would
    /// otherwise stomp each other's root directory.
    macro_rules! with_temp_file_root {
        ($body:block) => {{
            let _guard = crate::state::test_support::lock_test();
            let dir = tempfile::tempdir().unwrap();
            file::reset_with_root(dir.path());
            let result = $body;
            file::reset();
            result
        }};
    }

    #[test]
    fn save_picture_writes_a_valid_bitmap() {
        with_temp_file_root!({
            save_picture(&picture_variant(2, 3), &VBVariant::from_string("out.bmp")).unwrap();

            let bytes = std::fs::read(file::get_root().join("out.bmp")).unwrap();
            assert_eq!(&bytes[0..2], b"BM");
            assert_eq!(bytes.len(), 54 + 8 * 3); // header + 3 padded rows of 2 px
            assert_eq!(
                u32::from_le_bytes(bytes[2..6].try_into().unwrap()),
                bytes.len() as u32
            );
            assert_eq!(u32::from_le_bytes(bytes[10..14].try_into().unwrap()), 54);
            assert_eq!(u32::from_le_bytes(bytes[14..18].try_into().unwrap()), 40);
            assert_eq!(i32::from_le_bytes(bytes[18..22].try_into().unwrap()), 2); // width
            assert_eq!(i32::from_le_bytes(bytes[22..26].try_into().unwrap()), 3); // height
            assert_eq!(u16::from_le_bytes(bytes[26..28].try_into().unwrap()), 1); // planes
            assert_eq!(u16::from_le_bytes(bytes[28..30].try_into().unwrap()), 24); // bpp
                                                                                   // Every pixel is white (0xFF), padding bytes are zeroed.
            assert!(bytes[54..60].iter().all(|&b| b == 0xFF));
            assert!(bytes[60..62].iter().all(|&b| b == 0));
        });
    }

    #[test]
    fn save_picture_overwrites_an_existing_file_without_warning() {
        with_temp_file_root!({
            std::fs::write(file::get_root().join("out.bmp"), b"stale data").unwrap();

            save_picture(&picture_variant(1, 1), &VBVariant::from_string("out.bmp")).unwrap();

            let bytes = std::fs::read(file::get_root().join("out.bmp")).unwrap();
            assert_eq!(bytes.len(), 54 + 4); // one padded row of 1 px
            assert_eq!(&bytes[0..2], b"BM");
        });
    }

    #[test]
    fn save_picture_accepts_absolute_paths() {
        with_temp_file_root!({
            let dir = file::get_root();
            let target = dir.join("sub").join("out.bmp");
            std::fs::create_dir(dir.join("sub")).unwrap();

            save_picture(
                &picture_variant(4, 4),
                &VBVariant::from_string(target.to_str().unwrap()),
            )
            .unwrap();

            assert!(target.exists());
        });
    }

    #[test]
    fn save_picture_nothing_raises_object_variable_not_set() {
        with_temp_file_root!({
            let error =
                save_picture(&VBVariant::Nothing, &VBVariant::from_string("out.bmp")).unwrap_err();
            assert_eq!(error.number, err_number::OBJECT_VARIABLE_NOT_SET);
        });
    }

    #[test]
    fn save_picture_non_object_raises_type_mismatch() {
        with_temp_file_root!({
            let error = save_picture(
                &VBVariant::from_integer(42),
                &VBVariant::from_string("out.bmp"),
            )
            .unwrap_err();
            assert_eq!(error.number, err_number::TYPE_MISMATCH);
        });
    }

    #[test]
    fn save_picture_null_filename_raises_invalid_use_of_null() {
        with_temp_file_root!({
            let error = save_picture(&picture_variant(1, 1), &VBVariant::Null).unwrap_err();
            assert_eq!(error.number, err_number::INVALID_USE_OF_NULL);
        });
    }

    #[test]
    fn save_picture_empty_filename_raises_path_file_access_error() {
        with_temp_file_root!({
            let error =
                save_picture(&picture_variant(1, 1), &VBVariant::from_string("")).unwrap_err();
            assert_eq!(error.number, err_number::PATH_FILE_ACCESS_ERROR);
        });
    }

    #[test]
    fn save_picture_creates_missing_parent_directories_like_output_mode() {
        with_temp_file_root!({
            save_picture(
                &picture_variant(1, 1),
                &VBVariant::from_string("new_dir/out.bmp"),
            )
            .unwrap();

            assert!(file::get_root().join("new_dir").join("out.bmp").exists());
        });
    }
}
