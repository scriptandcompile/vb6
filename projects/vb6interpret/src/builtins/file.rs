//! VB6 file function registry.
//!
//! One [`typed_builtin!`](crate::typed_builtin) entry per file function.
//!
//! Every parameter stays a raw Variant by design: the `LoadRes`-style
//! strictness here is deliberate — path parameters accept only actual
//! strings and file-number parameters only actual Byte/Integer/Longs, each
//! rejection reporting "Type mismatch in <Function>" (13) rather than the
//! boundary's coercion semantics. An omitted argument acts as `Empty`
//! where the body says so (`Dir`, `FreeFile`, `CurDir`).
//!
//! Not registered: the file *statements* (`Open`, `Close`, `Print#`,
//! `Write#`, `Input#`, `Line Input#`, `Get#`, `Put#`, `Lock`, `Unlock`,
//! `Seek`, `Width`, `Reset`) are executed directly by `exec::file_io` and
//! `exec::print`.

use super::{Builtin, Registry};
use crate::typed_builtin;
use vb6runtime::library::file::ch_dir::chdir;
use vb6runtime::library::file::ch_drive::chdrive;
use vb6runtime::library::file::curdir::curdir;
use vb6runtime::library::file::curdir_dollar::curdir_dollar;
use vb6runtime::library::file::dir::dir;
use vb6runtime::library::file::eof::eof;
use vb6runtime::library::file::fileattr::fileattr;
use vb6runtime::library::file::filecopy::file_copy;
use vb6runtime::library::file::filedatetime::file_datetime;
use vb6runtime::library::file::filelen::file_len;
use vb6runtime::library::file::freefile::free_file;
use vb6runtime::library::file::getattr::getattr;
use vb6runtime::library::file::input::input;
use vb6runtime::library::file::kill::kill;
use vb6runtime::library::file::loc::loc;
use vb6runtime::library::file::lof::lof;
use vb6runtime::library::file::mkdir::mkdir;
use vb6runtime::library::file::name::name_statement;
use vb6runtime::library::file::rmdir::rmdir;
use vb6runtime::library::file::seek::seek;
use vb6runtime::library::file::setattr::setattr;
use vb6runtime::VBVariant;

/// Register the file functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    registry.insert(typed_builtin!("dir", 0, 2,
        (pathname: opt_variant, attributes: opt_variant),
        dir(pathname, attributes)));
    registry.insert(
        typed_builtin!("freefile", 0, 1, (range_number: opt_variant),
        free_file(range_number).map(VBVariant::from)),
    );
    registry.insert(typed_builtin!("eof", 1, 1, (file_number: variant),
        eof(file_number).map(VBVariant::from)));
    registry.insert(typed_builtin!("lof", 1, 1, (file_number: variant),
        lof(file_number).map(VBVariant::from)));
    registry.insert(typed_builtin!("loc", 1, 1, (file_number: variant),
        loc(file_number).map(VBVariant::from)));
    registry.insert(typed_builtin!("seek", 1, 1, (file_number: variant),
        seek(file_number).map(VBVariant::from)));
    registry.insert(typed_builtin!("filelen", 1, 1, (pathname: variant),
        file_len(pathname).map(VBVariant::from)));
    registry.insert(typed_builtin!("fileattr", 2, 2,
        (file_number: variant, return_type: variant),
        fileattr(file_number, return_type).map(VBVariant::from)));
    registry.insert(typed_builtin!("filedatetime", 1, 1, (pathname: variant),
        file_datetime(pathname)));
    registry.insert(typed_builtin!("curdir", 0, 1, (drive: opt_variant),
        curdir(drive)));
    registry.insert(typed_builtin!("curdir$", 0, 1, (drive: opt_variant),
        curdir_dollar(drive)));
    registry.insert(typed_builtin!("getattr", 1, 1, (pathname: variant),
        getattr(pathname).map(VBVariant::from)));
    registry.insert(typed_builtin!("setattr", 2, 2,
        (pathname: variant, attributes: variant), {
        setattr(pathname, attributes)?;
        Ok(VBVariant::Empty)
    }));
    registry.insert(typed_builtin!("kill", 1, 1, (pathname: variant), {
        kill(pathname)?;
        Ok(VBVariant::Empty)
    }));
    registry.insert(typed_builtin!("filecopy", 2, 2,
        (source: variant, destination: variant), {
        file_copy(source, destination)?;
        Ok(VBVariant::Empty)
    }));
    registry.insert(typed_builtin!("name", 2, 2,
        (old_pathname: variant, new_pathname: variant), {
        name_statement(old_pathname, new_pathname)?;
        Ok(VBVariant::Empty)
    }));
    registry.insert(typed_builtin!("mkdir", 1, 1, (path: variant), {
        mkdir(path)?;
        Ok(VBVariant::Empty)
    }));
    registry.insert(typed_builtin!("rmdir", 1, 1, (path: variant), {
        rmdir(path)?;
        Ok(VBVariant::Empty)
    }));
    registry.insert(typed_builtin!("chdir", 1, 1, (path: variant), {
        chdir(path)?;
        Ok(VBVariant::Empty)
    }));
    registry.insert(typed_builtin!("chdrive", 1, 1, (drive: variant), {
        chdrive(drive)?;
        Ok(VBVariant::Empty)
    }));
    registry.insert(typed_builtin!("input", 2, 2,
        (number_chars: variant, file_number: variant),
        input(number_chars, file_number)));
}
