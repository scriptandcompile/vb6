//! StdPicture runtime representation.
//
// The StdPicture object is the VB6 standard class that represents a picture,
// implementing the IPicture OLE automation interface. It is used as the type
// for the Picture property of controls (PictureBox, Image, Form) and is
// returned by LoadPicture and LoadResPicture functions.

use crate::value::VBObject;
use std::any::Any;
use std::fmt;

/// VB6 StdPicture object representation.
///
/// This is the runtime equivalent of the VB6 StdPicture class, which implements
/// the IPicture OLE automation interface. It can be assigned to the Picture
/// property of controls (PictureBox, Image, Form) and supports the public
/// VB6-accessible data and procedures.
#[derive(Debug)]
pub struct StdPicture {
    /// Picture width in pixels.
    width: i32,
    /// Picture height in pixels.
    height: i32,
    /// Handle to the picture (HBITMAP or similar). None if not loaded from a
    /// file with a handle.
    handle: Option<u32>,
}

impl StdPicture {
    /// Create a new empty StdPicture with the given dimensions.
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            handle: None,
        }
    }

    /// Get the picture width in pixels.
    pub fn width(&self) -> i32 {
        self.width
    }

    /// Get the picture height in pixels.
    pub fn height(&self) -> i32 {
        self.height
    }

    /// Get the picture handle (HBITMAP or similar), if any.
    pub fn handle(&self) -> Option<u32> {
        self.handle
    }

    /// Set the picture handle.
    pub fn set_handle(&mut self, handle: u32) {
        self.handle = Some(handle);
    }
}

impl fmt::Display for StdPicture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StdPicture(width={}, height={})",
            self.width, self.height
        )
    }
}

impl VBObject for StdPicture {
    fn type_name(&self) -> &str {
        "StdPicture"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn VBObject> {
        Box::new(StdPicture {
            width: self.width,
            height: self.height,
            handle: self.handle,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::VBVariant;

    #[test]
    fn stdpicture_type_name() {
        let pic = StdPicture::new(100, 200);
        assert_eq!(pic.type_name(), "StdPicture");
    }

    #[test]
    fn stdpicture_clone() {
        let original = StdPicture::new(100, 200);
        let cloned = original.clone_box();
        let cloned_pic = cloned.as_any().downcast_ref::<StdPicture>().unwrap();
        assert_eq!(cloned_pic.width(), 100);
        assert_eq!(cloned_pic.height(), 200);
    }

    #[test]
    fn stdpicture_as_variant() {
        let pic = StdPicture::new(100, 200);
        let variant = VBVariant::from_object(Box::new(pic));
        let retrieved = variant.as_object().unwrap();
        assert_eq!(retrieved.type_name(), "StdPicture");
    }
}
