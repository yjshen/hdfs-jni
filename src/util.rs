use libc::c_char;
use std::ffi::CStr;
use std::str;

use crate::dfs::HdfsFs;
use crate::err::HdfsErr;
use crate::native::*;

#[macro_export]
macro_rules! to_raw {
    ($str:expr) => {{
        let c_str = std::ffi::CString::new($str).unwrap();
        c_str.into_raw()
    }};
}

pub fn chars_to_str<'a>(chars: *const c_char) -> &'a str {
    let slice = unsafe { CStr::from_ptr(chars) }.to_bytes();
    str::from_utf8(slice).unwrap()
}

#[macro_export]
macro_rules! b2i {
    ($b:expr) => {{
        if $b {
            1
        } else {
            0
        }
    }};
}

/// Hdfs Utility
pub struct HdfsUtil;

/// HDFS Utility
impl HdfsUtil {
    /// Copy file from one filesystem to another.
    ///
    /// #### Params
    /// * ```srcFS``` - The handle to source filesystem.
    /// * ```src``` - The path of source file.
    /// * ```dstFS``` - The handle to destination filesystem.
    /// * ```dst``` - The path of destination file.
    pub fn copy(
        src_fs: &HdfsFs<'_>,
        src: &str,
        dst_fs: &HdfsFs<'_>,
        dst: &str,
    ) -> Result<bool, HdfsErr> {
        let res = unsafe {
            hdfsCopy(
                src_fs.raw(),
                to_raw!(src),
                dst_fs.raw(),
                to_raw!(dst),
            )
        };

        if res == 0 {
            Ok(true)
        } else {
            Err(HdfsErr::Unknown)
        }
    }

    /// Move file from one filesystem to another.
    ///
    /// #### Params
    /// * ```srcFS``` - The handle to source filesystem.
    /// * ```src``` - The path of source file.
    /// * ```dstFS``` - The handle to destination filesystem.
    /// * ```dst``` - The path of destination file.
    pub fn mv(
        src_fs: &HdfsFs<'_>,
        src: &str,
        dst_fs: &HdfsFs<'_>,
        dst: &str,
    ) -> Result<bool, HdfsErr> {
        let res = unsafe {
            hdfsMove(
                src_fs.raw(),
                to_raw!(src),
                dst_fs.raw(),
                to_raw!(dst),
            )
        };

        if res == 0 {
            Ok(true)
        } else {
            Err(HdfsErr::Unknown)
        }
    }
}
