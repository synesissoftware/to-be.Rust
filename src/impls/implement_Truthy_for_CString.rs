// impls/implement_Truthy_for_CString.rs

use crate::truthy::Truthy;

use std::ffi::CString;

impl Truthy for CString {
    fn is_truthy(&self) -> Option<bool> {
        match self.to_str() {
            Err(_) => None,
            Ok(c) => crate::parse::string_is_truthy(c),
        }
    }
}

impl Truthy for &CString {
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

    use std::ffi::CString;


    #[test]
    fn TEST_CString_Truthy() {
        // is_falsey
        {
            assert_eq!(false, CString::new("").unwrap().is_falsey());
            assert_eq!(false, CString::new("Copyright ©").unwrap().is_falsey());

            assert_eq!(true, CString::new("0").unwrap().is_falsey());
            assert_eq!(true, CString::new("false").unwrap().is_falsey());
            assert_eq!(true, CString::new(" FALSE").unwrap().is_falsey());
            assert_eq!(true, CString::new("False").unwrap().is_falsey());
            assert_eq!(true, CString::new("FaLSe").unwrap().is_falsey());
            assert_eq!(true, CString::new("no").unwrap().is_falsey());
            assert_eq!(true, CString::new("No ").unwrap().is_falsey());
            assert_eq!(true, CString::new("NO").unwrap().is_falsey());
            assert_eq!(true, CString::new(" Off ").unwrap().is_falsey());
            assert_eq!(true, CString::new("off").unwrap().is_falsey());
            assert_eq!(true, CString::new("OFF").unwrap().is_falsey());

            assert_eq!(false, CString::new("1").unwrap().is_falsey());
            assert_eq!(false, CString::new("true").unwrap().is_falsey());
            assert_eq!(false, CString::new("TRUE").unwrap().is_falsey());
            assert_eq!(false, CString::new("True").unwrap().is_falsey());
            assert_eq!(false, CString::new("tRuE").unwrap().is_falsey());
            assert_eq!(false, CString::new("yes").unwrap().is_falsey());
            assert_eq!(false, CString::new(" YES").unwrap().is_falsey());
            assert_eq!(false, CString::new("Yes   ").unwrap().is_falsey());
            assert_eq!(false, CString::new("yEs").unwrap().is_falsey());
        }

        // is_truey
        {
            assert_eq!(false, CString::new("").unwrap().is_truey());
            assert_eq!(false, CString::new("Copyright ©").unwrap().is_truey());

            assert_eq!(false, CString::new("0").unwrap().is_truey());
            assert_eq!(false, CString::new("false").unwrap().is_truey());
            assert_eq!(false, CString::new(" FALSE").unwrap().is_truey());
            assert_eq!(false, CString::new("False").unwrap().is_truey());
            assert_eq!(false, CString::new("FaLSe").unwrap().is_truey());
            assert_eq!(false, CString::new("no").unwrap().is_truey());
            assert_eq!(false, CString::new("No ").unwrap().is_truey());
            assert_eq!(false, CString::new("NO").unwrap().is_truey());
            assert_eq!(false, CString::new(" Off ").unwrap().is_truey());
            assert_eq!(false, CString::new("off").unwrap().is_truey());
            assert_eq!(false, CString::new("OFF").unwrap().is_truey());

            assert_eq!(true, CString::new("1").unwrap().is_truey());
            assert_eq!(true, CString::new("true").unwrap().is_truey());
            assert_eq!(true, CString::new("TRUE").unwrap().is_truey());
            assert_eq!(true, CString::new("True").unwrap().is_truey());
            assert_eq!(true, CString::new("tRuE").unwrap().is_truey());
            assert_eq!(true, CString::new("yes").unwrap().is_truey());
            assert_eq!(true, CString::new(" YES").unwrap().is_truey());
            assert_eq!(true, CString::new("Yes   ").unwrap().is_truey());
            assert_eq!(true, CString::new("yEs").unwrap().is_truey());
        }

        // is_truthy
        {
            assert_eq!(None, CString::new("").unwrap().is_truthy());
            assert_eq!(None, CString::new("Copyright ©").unwrap().is_truthy());

            assert_eq!(Some(false), CString::new("0").unwrap().is_truthy());
            assert_eq!(Some(false), CString::new("false").unwrap().is_truthy());
            assert_eq!(Some(false), CString::new(" FALSE").unwrap().is_truthy());
            assert_eq!(Some(false), CString::new("False").unwrap().is_truthy());
            assert_eq!(Some(false), CString::new("FaLSe").unwrap().is_truthy());
            assert_eq!(Some(false), CString::new("no").unwrap().is_truthy());
            assert_eq!(Some(false), CString::new("No ").unwrap().is_truthy());
            assert_eq!(Some(false), CString::new("NO").unwrap().is_truthy());
            assert_eq!(Some(false), CString::new(" Off ").unwrap().is_truthy());
            assert_eq!(Some(false), CString::new("off").unwrap().is_truthy());
            assert_eq!(Some(false), CString::new("OFF").unwrap().is_truthy());

            assert_eq!(Some(true), CString::new("1").unwrap().is_truthy());
            assert_eq!(Some(true), CString::new("true").unwrap().is_truthy());
            assert_eq!(Some(true), CString::new("TRUE").unwrap().is_truthy());
            assert_eq!(Some(true), CString::new("True").unwrap().is_truthy());
            assert_eq!(Some(true), CString::new("tRuE").unwrap().is_truthy());
            assert_eq!(Some(true), CString::new("yes").unwrap().is_truthy());
            assert_eq!(Some(true), CString::new(" YES").unwrap().is_truthy());
            assert_eq!(Some(true), CString::new("Yes   ").unwrap().is_truthy());
            assert_eq!(Some(true), CString::new("yEs").unwrap().is_truthy());
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //
