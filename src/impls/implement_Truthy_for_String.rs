// impls/implement_Truthy_for_String.rs

use crate::truthy::Truthy;

impl Truthy for String {
    fn is_truthy(&self) -> Option<bool> {
        crate::parse::string_is_truthy(self.as_str())
    }
}

impl Truthy for &String {
    fn is_truthy(&self) -> Option<bool> {
        crate::parse::string_is_truthy(self.as_str())
    }
}


#[cfg(test)]
mod tests {
    #![allow(clippy::bool_assert_comparison)]
    #![allow(non_snake_case)]

    use crate::truthy::Truthy as _;


    #[test]
    fn TEST_String_Truthy() {
        // is_falsey
        {
            assert_eq!(false, String::from("").is_falsey());

            assert_eq!(true, String::from("0").is_falsey());
            assert_eq!(true, String::from("false").is_falsey());
            assert_eq!(true, String::from(" FALSE").is_falsey());
            assert_eq!(true, String::from("False").is_falsey());
            assert_eq!(true, String::from("FaLSe").is_falsey());
            assert_eq!(true, String::from("no").is_falsey());
            assert_eq!(true, String::from("No ").is_falsey());
            assert_eq!(true, String::from("NO").is_falsey());
            assert_eq!(true, String::from(" Off ").is_falsey());
            assert_eq!(true, String::from("off").is_falsey());
            assert_eq!(true, String::from("OFF").is_falsey());

            assert_eq!(false, String::from("1").is_falsey());
            assert_eq!(false, String::from("true").is_falsey());
            assert_eq!(false, String::from("TRUE").is_falsey());
            assert_eq!(false, String::from("True").is_falsey());
            assert_eq!(false, String::from("tRuE").is_falsey());
            assert_eq!(false, String::from("yes").is_falsey());
            assert_eq!(false, String::from(" YES").is_falsey());
            assert_eq!(false, String::from("Yes   ").is_falsey());
            assert_eq!(false, String::from("yEs").is_falsey());
        }

        // is_truey
        {
            assert_eq!(false, String::from("").is_truey());

            assert_eq!(false, String::from("0").is_truey());
            assert_eq!(false, String::from("false").is_truey());
            assert_eq!(false, String::from(" FALSE").is_truey());
            assert_eq!(false, String::from("False").is_truey());
            assert_eq!(false, String::from("FaLSe").is_truey());
            assert_eq!(false, String::from("no").is_truey());
            assert_eq!(false, String::from("No ").is_truey());
            assert_eq!(false, String::from("NO").is_truey());
            assert_eq!(false, String::from(" Off ").is_truey());
            assert_eq!(false, String::from("off").is_truey());
            assert_eq!(false, String::from("OFF").is_truey());

            assert_eq!(true, String::from("1").is_truey());
            assert_eq!(true, String::from("true").is_truey());
            assert_eq!(true, String::from("TRUE").is_truey());
            assert_eq!(true, String::from("True").is_truey());
            assert_eq!(true, String::from("tRuE").is_truey());
            assert_eq!(true, String::from("yes").is_truey());
            assert_eq!(true, String::from(" YES").is_truey());
            assert_eq!(true, String::from("Yes   ").is_truey());
            assert_eq!(true, String::from("yEs").is_truey());
        }

        // is_truthy
        {
            assert_eq!(None, String::from("").is_truthy());

            assert_eq!(Some(false), String::from("0").is_truthy());
            assert_eq!(Some(false), String::from("false").is_truthy());
            assert_eq!(Some(false), String::from(" FALSE").is_truthy());
            assert_eq!(Some(false), String::from("False").is_truthy());
            assert_eq!(Some(false), String::from("FaLSe").is_truthy());
            assert_eq!(Some(false), String::from("no").is_truthy());
            assert_eq!(Some(false), String::from("No ").is_truthy());
            assert_eq!(Some(false), String::from("NO").is_truthy());
            assert_eq!(Some(false), String::from(" Off ").is_truthy());
            assert_eq!(Some(false), String::from("off").is_truthy());
            assert_eq!(Some(false), String::from("OFF").is_truthy());

            assert_eq!(Some(true), String::from("1").is_truthy());
            assert_eq!(Some(true), String::from("true").is_truthy());
            assert_eq!(Some(true), String::from("TRUE").is_truthy());
            assert_eq!(Some(true), String::from("True").is_truthy());
            assert_eq!(Some(true), String::from("tRuE").is_truthy());
            assert_eq!(Some(true), String::from("yes").is_truthy());
            assert_eq!(Some(true), String::from(" YES").is_truthy());
            assert_eq!(Some(true), String::from("Yes   ").is_truthy());
            assert_eq!(Some(true), String::from("yEs").is_truthy());
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //
