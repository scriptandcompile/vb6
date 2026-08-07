//! VB6 string functions.
//!
//! Each implemented function lives in its own file (e.g. `len.rs`) and is
//! re-exported here so it is callable as `string::len(...)` rather than
//! `string::len::len(...)`. Documentation-only stubs stay as plain modules.

pub mod ansi;

pub mod asc;
pub use asc::asc;

pub mod ascb;
pub use ascb::ascb;

pub mod ascw;
pub use ascw::ascw;

pub mod chr;
pub use chr::chr;

pub mod chr_dollar;
//pub use chr_dollar::chr_dollar;

pub mod chrb;
//pub use chrb::chrb;

pub mod chrb_dollar;
//pub use chrb_dollar::chrb_dollar;
pub mod chrw;
pub use chrw::chrw;

pub mod chrw_dollar;
//pub use chrw_dollar::chrw_dollar;

pub mod format;
//pub use format::format;

pub mod format_dollar;
pub use format_dollar::format_dollar;

pub mod formatcurrency;
//pub use formatcurrency::formatcurrency;

pub mod formatdatetime;
//pub use formatdatetime::formatdatetime;

pub mod formatnumber;
//pub use formatnumber::formatnumber;

pub mod formatpercent;
//pub use formatpercent::formatpercent;

pub mod instr;
pub use instr::instr;

pub mod instrrev;
pub use instrrev::instrrev;

pub mod lcase;
pub use lcase::lcase;

pub mod lcase_dollar;
//pub use lcase_dollar::lcase_dollar;

pub mod left;
pub use left::left;

pub mod left_dollar;
//pub use left_dollar::left_dollar;

pub mod leftb;
//pub use leftb::leftb;

pub mod leftb_dollar;
//pub use leftb_dollar::leftb_dollar;

pub mod len;
pub use len::len;

pub mod lenb;
//pub use lenb::lenb;

pub mod ltrim;
pub use ltrim::ltrim;

pub mod ltrim_dollar;
//pub use ltrim_dollar::ltrim_dollar;

pub mod mid;
pub use mid::mid;

pub mod mid_dollar;
//pub use mid_dollar::mid_dollar;

pub mod midb;
//pub use midb::midb;

pub mod midb_dollar;
//pub use midb_dollar::midb_dollar;

pub mod replace;
//pub use replace::replace;

pub mod right;
pub use right::right;

pub mod right_dollar;
//pub use right_dollar::right_dollar;

pub mod rightb;
//pub use rightb::rightb;

pub mod rightb_dollar;
//pub use rightb_dollar::rightb_dollar;

pub mod rtrim;
pub use rtrim::rtrim;

pub mod rtrim_dollar;
//pub use rtrim_dollar::rtrim_dollar;

pub mod space;
pub use space::space;

pub mod space_dollar;
//pub use space_dollar::space_dollar;

pub mod str;
//pub use str::str;

pub mod str_dollar;
//pub use str_dollar::str_dollar;

pub mod strcomp;
//pub use strcomp::strcomp;

pub mod strconv;
//pub use strconv::strconv;

pub mod string_function;
//pub use string_function::string_function;

pub mod strreverse;
pub use strreverse::strreverse;

pub mod trim;
pub use trim::trim;

pub mod trim_dollar;
//pub use trim_dollar::trim_dollar;

pub mod ucase;
pub use ucase::ucase;

pub mod ucase_dollar;
//pub use ucase_dollar::ucase_dollar;
