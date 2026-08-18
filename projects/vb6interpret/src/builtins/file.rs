//! VB6 file function registry.
//!
//! One [`Builtin`](super::Builtin) entry per file function, each wrapping
//! the typed `vb6runtime::library::file` implementation.

use super::{Builtin, Registry};
use crate::builtin;
use vb6core::error::VBResult;
use vb6runtime::library::file as filefn;
use vb6runtime::VBVariant;

/// Register the file functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    registry.insert(builtin!("dir", 0, 2, |args| {
        let pathname = args.first().cloned().unwrap_or(VBVariant::Empty);
        let attributes: i16 = args
            .get(1)
            .and_then(|v| v.as_i32().ok())
            .map(|n| n as i16)
            .unwrap_or(0);
        filefn::dir::dir(pathname, attributes)
    }));
    registry.insert(builtin!("freefile", 0, 1, |args| {
        let range = args.first().cloned().unwrap_or(VBVariant::Empty);
        filefn::freefile::free_file(range).map(VBVariant::from)
    }));
    registry.insert(builtin!("eof", 1, 1, |args| {
        filefn::eof::eof(args[0].clone()).map(VBVariant::from)
    }));
    registry.insert(builtin!("lof", 1, 1, |args| {
        filefn::lof::lof(args[0].clone()).map(VBVariant::from)
    }));
    registry.insert(builtin!("loc", 1, 1, |args| {
        filefn::loc::loc(args[0].clone()).map(VBVariant::from)
    }));
    registry.insert(builtin!("filelen", 1, 1, |args| {
        filefn::filelen::file_len(args[0].clone()).map(VBVariant::from)
    }));
    registry.insert(builtin!("fileattr", 2, 2, |args| {
        filefn::fileattr::fileattr(args[0].clone(), args[1].clone()).map(VBVariant::from)
    }));
    registry.insert(builtin!("filedatetime", 1, 1, |args| {
        filefn::filedatetime::file_datetime(args[0].clone())
    }));
    registry.insert(builtin!("seek", 1, 1, |args| {
        filefn::seek::seek(args[0].clone()).map(VBVariant::from)
    }));
    registry.insert(builtin!("curdir", 0, 1, |args| {
        let drive = args.first().cloned().unwrap_or(VBVariant::Empty);
        filefn::curdir::curdir(drive)
    }));
    registry.insert(builtin!("curdir$", 0, 1, |args| {
        let drive = args.first().cloned().unwrap_or(VBVariant::Empty);
        filefn::curdir_dollar::curdir_dollar(drive)
    }));
    registry.insert(builtin!("getattr", 1, 1, |args| {
        filefn::getattr::getattr(args[0].clone()).map(VBVariant::from)
    }));
    registry.insert(builtin!("setattr", 2, 2, |args| {
        filefn::setattr::setattr(args[0].clone(), args[1].clone())?;
        Ok(VBVariant::Empty)
    }));
    registry.insert(builtin!("kill", 1, 1, |args| {
        filefn::kill::kill(args[0].clone())?;
        Ok(VBVariant::Empty)
    }));
    registry.insert(builtin!("filecopy", 2, 2, |args| {
        filefn::filecopy::file_copy(args[0].clone(), args[1].clone())?;
        Ok(VBVariant::Empty)
    }));
    registry.insert(builtin!("name", 2, 2, |args| {
        filefn::name::name_statement(args[0].clone(), args[1].clone())?;
        Ok(VBVariant::Empty)
    }));
    registry.insert(builtin!("mkdir", 1, 1, |args| {
        filefn::mkdir::mkdir(args[0].clone())?;
        Ok(VBVariant::Empty)
    }));
    registry.insert(builtin!("rmdir", 1, 1, |args| {
        filefn::rmdir::rmdir(args[0].clone())?;
        Ok(VBVariant::Empty)
    }));
    registry.insert(builtin!("chdir", 1, 1, |args| {
        filefn::ch_dir::chdir(args[0].clone())?;
        Ok(VBVariant::Empty)
    }));
    registry.insert(builtin!("chdrive", 1, 1, |args| {
        filefn::ch_drive::chdrive(args[0].clone())?;
        Ok(VBVariant::Empty)
    }));
}
