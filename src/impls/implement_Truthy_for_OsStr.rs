// impls/implement_Truthy_for_OsStr.rs

use crate::truthy::Truthy;

use std::ffi::OsStr;

impl Truthy for OsStr {
    fn is_truthy(&self) -> Option<bool> {
        crate::parse::os_string_is_truthy(self)
    }
}

impl Truthy for &OsStr {
    fn is_truthy(&self) -> Option<bool> {
        crate::parse::os_string_is_truthy(*self)
    }
}


#[cfg(test)]
mod tests {
    #![allow(clippy::bool_assert_comparison)]
    #![allow(non_snake_case)]

    use crate::truthy::Truthy as _;

    use std::ffi::OsStr;


    #[test]
    fn TEST_OsStr_Truthy() {
        assert_eq!(Some(true), OsStr::new("yes").is_truthy());
        assert_eq!(Some(false), OsStr::new("no").is_truthy());
        assert_eq!(None, OsStr::new("").is_truthy());
    }

    #[cfg(unix)]
    #[test]
    fn TEST_OsStr_Truthy_non_utf8() {
        use std::os::unix::ffi::OsStrExt;

        let non_utf8 = OsStr::from_bytes(&[0xff]);
        assert_eq!(None, non_utf8.is_truthy());
    }
}


// ///////////////////////////// end of file //////////////////////////// //
