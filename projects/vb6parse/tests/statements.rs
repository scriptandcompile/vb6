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

#[path = "statements/file_operations/get.rs"]
mod get;

#[path = "statements/file_operations/kill.rs"]
mod kill;

#[path = "statements/file_operations/input.rs"]
mod input;

#[path = "statements/file_operations/line_input.rs"]
mod line_input;

#[path = "statements/file_operations/name.rs"]
mod name;

#[path = "statements/file_operations/open.rs"]
mod open;

#[path = "statements/file_operations/lock.rs"]
mod lock;

#[path = "statements/file_operations/print.rs"]
mod print;

#[path = "statements/file_operations/reset.rs"]
mod reset;
