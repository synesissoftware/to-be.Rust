// impls/implement_Truthy_for_CStr.rs

use crate::truthy::Truthy;

use std::ffi::CStr;

impl Truthy for CStr {
    fn is_truthy(&self) -> Option<bool> {
        match self.to_str() {
            Err(_) => None,
            Ok(c) => crate::parse::string_is_truthy(c),
        }
    }
}

impl Truthy for &CStr {
    fn is_truthy(&self) -> Option<bool> {
        match self.to_str() {
            Err(_) => None,
            Ok(c) => crate::parse::string_is_truthy(c),
        }
    }
}


#[cfg(test)]
mod tests {
    #![allow(clippy::bool_assert_comparison)]
    #![allow(non_snake_case)]

    use crate::truthy::Truthy as _;

    use std::ffi::CStr;


    #[test]
    fn TEST_CStr_Truthy() {
        let yes = CStr::from_bytes_with_nul(b"yes\0").unwrap();
        let invalid = CStr::from_bytes_with_nul(b"\xff\0").unwrap();

        assert_eq!(Some(true), yes.is_truthy());
        assert_eq!(None, invalid.is_truthy());
    }
}


// ///////////////////////////// end of file //////////////////////////// //
