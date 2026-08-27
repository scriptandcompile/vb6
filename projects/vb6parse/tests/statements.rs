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

#[path = "statements/file_operations/unlock.rs"]
mod unlock;

#[path = "statements/file_operations/print.rs"]
mod print;

#[path = "statements/file_operations/seek.rs"]
mod seek;

#[path = "statements/file_operations/width.rs"]
mod width;

#[path = "statements/file_operations/write.rs"]
mod write;

#[path = "statements/filesystem/chdir.rs"]
mod chdir;

#[path = "statements/filesystem/chdrive.rs"]
mod chdrive;

#[path = "statements/filesystem/mkdir.rs"]
mod mkdir;

#[path = "statements/filesystem/rmdir.rs"]
mod rmdir;

#[path = "statements/filesystem/setattr.rs"]
mod setattr;

#[path = "statements/objects/call.rs"]
mod call;

#[path = "statements/objects/events.rs"]
mod events;

#[path = "statements/objects/set.rs"]
mod set;

#[path = "statements/objects/with_block.rs"]
mod with_block;

#[path = "statements/runtime_state/date.rs"]
mod date;

#[path = "statements/runtime_state/error.rs"]
mod error;
