// Statement parsing tests organized by statement category

#[path = "statements/declarations/arrays.rs"]
mod arrays;

#[path = "statements/declarations/erase.rs"]
mod erase;

#[path = "statements/declarations/variables.rs"]
mod variables;

#[path = "statements/control_flow/exit.rs"]
mod exit;

#[path = "statements/control_flow/jump.rs"]
mod jump;

#[path = "statements/control_flow/resume.rs"]
mod resume;

#[path = "statements/file_operations/close.rs"]
mod close;

#[path = "statements/file_operations/filecopy.rs"]
mod filecopy;
