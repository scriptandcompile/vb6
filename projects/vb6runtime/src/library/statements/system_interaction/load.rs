//! VB6 Load statement syntax:
//! - Load object
//!
//! Loads a form or control into memory.
//!
//! The Load statement syntax has this part:
//!
//! | Part          | Description |
//! |---------------|-------------|
//! | object        | Required. An object expression that evaluates to a Form or control. |
//!
//! Remarks:
//! - When Visual Basic loads a form, it sets the form's Visible property to False.
//! - After loading a form, you can use the Show method to make the form visible.
//! - The controls on a form aren't accessible until the form is loaded.
//! - Load is typically used with forms that aren't shown at startup or with control arrays.
//! - For control arrays, you must use Load to create controls at run time.
//! - When you load a control array element, Visual Basic automatically increases the array's
//!   upper bound to accommodate the new element.
//! - You can't load a control that doesn't already exist at design time.
//! - The Load event occurs when the form is loaded.
//!
//! ## Examples
//!
//! ```vb
//! Load Form1
//! Load frmDialog
//! Load txtControl(5)
//! Load MyForm
//! ```
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/load-statement)
