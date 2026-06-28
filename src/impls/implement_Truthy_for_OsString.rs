// impls/implement_Truthy_for_OsString.rs

use crate::truthy::Truthy;

use std::ffi::OsString;

impl Truthy for OsString {
    fn is_truthy(&self) -> Option<bool> {
        crate::parse::os_string_is_truthy(self)
    }
}

impl Truthy for &OsString {
    fn is_truthy(&self) -> Option<bool> {
        crate::parse::os_string_is_truthy(*self)
    }
}


#[cfg(test)]
mod tests {
    #![allow(clippy::bool_assert_comparison)]
    #![allow(non_snake_case)]

    use crate::truthy::Truthy as _;

    use std::ffi::OsString;


    #[test]
    fn TEST_OsString_Truthy() {
        // is_falsey
        {
            assert_eq!(false, OsString::from("").is_falsey());

            assert_eq!(true, OsString::from("0").is_falsey());
            assert_eq!(true, OsString::from("false").is_falsey());
            assert_eq!(true, OsString::from(" FALSE").is_falsey());
            assert_eq!(true, OsString::from("False").is_falsey());
            assert_eq!(true, OsString::from("FaLSe").is_falsey());
            assert_eq!(true, OsString::from("no").is_falsey());
            assert_eq!(true, OsString::from("No ").is_falsey());
            assert_eq!(true, OsString::from("NO").is_falsey());
            assert_eq!(true, OsString::from(" Off ").is_falsey());
            assert_eq!(true, OsString::from("off").is_falsey());
            assert_eq!(true, OsString::from("OFF").is_falsey());

            assert_eq!(false, OsString::from("1").is_falsey());
            assert_eq!(false, OsString::from("true").is_falsey());
            assert_eq!(false, OsString::from("TRUE").is_falsey());
            assert_eq!(false, OsString::from("True").is_falsey());
            assert_eq!(false, OsString::from("tRuE").is_falsey());
            assert_eq!(false, OsString::from("yes").is_falsey());
            assert_eq!(false, OsString::from(" YES").is_falsey());
            assert_eq!(false, OsString::from("Yes   ").is_falsey());
            assert_eq!(false, OsString::from("yEs").is_falsey());
        }

        // is_truey
        {
            assert_eq!(false, OsString::from("").is_truey());

            assert_eq!(false, OsString::from("0").is_truey());
            assert_eq!(false, OsString::from("false").is_truey());
            assert_eq!(false, OsString::from(" FALSE").is_truey());
            assert_eq!(false, OsString::from("False").is_truey());
            assert_eq!(false, OsString::from("FaLSe").is_truey());
            assert_eq!(false, OsString::from("no").is_truey());
            assert_eq!(false, OsString::from("No ").is_truey());
            assert_eq!(false, OsString::from("NO").is_truey());
            assert_eq!(false, OsString::from(" Off ").is_truey());
            assert_eq!(false, OsString::from("off").is_truey());
            assert_eq!(false, OsString::from("OFF").is_truey());

            assert_eq!(true, OsString::from("1").is_truey());
            assert_eq!(true, OsString::from("true").is_truey());
            assert_eq!(true, OsString::from("TRUE").is_truey());
            assert_eq!(true, OsString::from("True").is_truey());
            assert_eq!(true, OsString::from("tRuE").is_truey());
            assert_eq!(true, OsString::from("yes").is_truey());
            assert_eq!(true, OsString::from(" YES").is_truey());
            assert_eq!(true, OsString::from("Yes   ").is_truey());
            assert_eq!(true, OsString::from("yEs").is_truey());
        }

        // is_truthy
        {
            assert_eq!(None, OsString::from("").is_truthy());

            assert_eq!(Some(false), OsString::from("0").is_truthy());
            assert_eq!(Some(false), OsString::from("false").is_truthy());
            assert_eq!(Some(false), OsString::from(" FALSE").is_truthy());
            assert_eq!(Some(false), OsString::from("False").is_truthy());
            assert_eq!(Some(false), OsString::from("FaLSe").is_truthy());
            assert_eq!(Some(false), OsString::from("no").is_truthy());
            assert_eq!(Some(false), OsString::from("No ").is_truthy());
            assert_eq!(Some(false), OsString::from("NO").is_truthy());
            assert_eq!(Some(false), OsString::from(" Off ").is_truthy());
            assert_eq!(Some(false), OsString::from("off").is_truthy());
            assert_eq!(Some(false), OsString::from("OFF").is_truthy());

            assert_eq!(Some(true), OsString::from("1").is_truthy());
            assert_eq!(Some(true), OsString::from("true").is_truthy());
            assert_eq!(Some(true), OsString::from("TRUE").is_truthy());
            assert_eq!(Some(true), OsString::from("True").is_truthy());
            assert_eq!(Some(true), OsString::from("tRuE").is_truthy());
            assert_eq!(Some(true), OsString::from("yes").is_truthy());
            assert_eq!(Some(true), OsString::from(" YES").is_truthy());
            assert_eq!(Some(true), OsString::from("Yes   ").is_truthy());
            assert_eq!(Some(true), OsString::from("yEs").is_truthy());
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //
