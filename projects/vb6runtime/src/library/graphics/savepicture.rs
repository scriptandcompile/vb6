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
