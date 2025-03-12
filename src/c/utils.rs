use std::ffi::{c_char, CStr};

use snafu::ResultExt;

use super::error;
use super::error::RustResult;

#[macro_export]
macro_rules! ensure_not_null {
    ($ptr: expr, $msg: expr) => {
        snafu::ensure!(
            !$ptr.is_null(),
            $crate::c::error::NullPointerSnafu { detail: $msg }
        );
    };
}

#[macro_export]
macro_rules! catch_unwind {
    ($expr: expr, $msg: expr) => {
        let ret = std::panic::catch_unwind(std::panic::AssertUnwindSafe($expr));
        match ret {
            Ok(ret) => return ret,
            Err(_e) => return $crate::error::CatchPanicSnafu { detail: $msg }.fail(),
        }
    };
}

/// Convert a C string to a Rust string. Note: a clone will occur.
///
/// # Safety
///
/// Caller must ensure that `c_str` is a valid pointer to a C string.
pub unsafe fn convert_c_string(c_str: *const c_char) -> RustResult<String> {
    ensure_not_null!(c_str, "c_str");

    let c_str = unsafe { CStr::from_ptr(c_str) };
    c_str
        .to_str()
        .context(error::ConvertUtf8Snafu { detail: "c_str" })
        .map(|s| s.to_string())
}

/// Convert a C string to a Rust string. If the C string is null, return the
/// default value. Note: a clone will occur.
///
/// # Safety
///
/// Caller must ensure that `c_str` is a valid pointer to a C string.
pub unsafe fn convert_c_string_with_default(
    c_str: *const c_char,
    default: impl Into<String>,
) -> RustResult<String> {
    if c_str.is_null() {
        return Ok(default.into());
    }

    let c_str = unsafe { CStr::from_ptr(c_str) };
    c_str
        .to_str()
        .context(error::ConvertUtf8Snafu { detail: "c_str" })
        .map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::os::raw::c_char;

    #[test]
    fn test_convert_c_string() {
        let c_string = CString::new("hello").unwrap();
        let c_str_ptr = c_string.as_ptr();

        let result = unsafe { convert_c_string(c_str_ptr) };

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello");
    }

    #[test]
    fn test_convert_c_string_null_pointer() {
        let c_str_ptr: *const c_char = std::ptr::null();

        let result = unsafe { convert_c_string(c_str_ptr) };

        assert!(result.is_err());
    }

    #[test]
    fn test_convert_c_string_invalid_utf8() {
        // Create an invalid UTF-8 C string
        let bytes = vec![0xff, 0xff, 0xff];
        let c_string = CString::new(bytes).unwrap();
        let c_str_ptr = c_string.as_ptr();

        let result = unsafe { convert_c_string(c_str_ptr) };

        assert!(result.is_err());
    }
}
