// impls/implement_Truthy_for_str.rs

use crate::truthy::Truthy;

impl Truthy for &str {
    fn is_truthy(&self) -> Option<bool> {
        crate::parse::string_is_truthy(self)
    }
}

impl Truthy for &&str {
    fn is_truthy(&self) -> Option<bool> {
        crate::parse::string_is_truthy(self)
    }
}


#[cfg(test)]
mod tests {
    #![allow(clippy::bool_assert_comparison)]
    #![allow(non_snake_case)]

    use crate::truthy::Truthy as _;


    #[test]
    fn TEST_str_Truthy() {
        // is_falsey
        {
            assert_eq!(false, "".is_falsey());

            assert_eq!(true, "0".is_falsey());
            assert_eq!(true, "false".is_falsey());
            assert_eq!(true, " FALSE".is_falsey());
            assert_eq!(true, "False".is_falsey());
            assert_eq!(true, "FaLSe".is_falsey());
            assert_eq!(true, "no".is_falsey());
            assert_eq!(true, "No ".is_falsey());
            assert_eq!(true, "NO".is_falsey());
            assert_eq!(true, " Off ".is_falsey());
            assert_eq!(true, "off".is_falsey());
            assert_eq!(true, "OFF".is_falsey());

            assert_eq!(false, "1".is_falsey());
            assert_eq!(false, "true".is_falsey());
            assert_eq!(false, "TRUE".is_falsey());
            assert_eq!(false, "True".is_falsey());
            assert_eq!(false, "tRuE".is_falsey());
            assert_eq!(false, "yes".is_falsey());
            assert_eq!(false, " YES".is_falsey());
            assert_eq!(false, "Yes   ".is_falsey());
            assert_eq!(false, "yEs".is_falsey());
        }

        {
            assert_eq!(false, "".is_truey());

            assert_eq!(false, "0".is_truey());
            assert_eq!(false, "false".is_truey());
            assert_eq!(false, " FALSE".is_truey());
            assert_eq!(false, "False".is_truey());
            assert_eq!(false, "FaLSe".is_truey());
            assert_eq!(false, "no".is_truey());
            assert_eq!(false, "No ".is_truey());
            assert_eq!(false, "NO".is_truey());
            assert_eq!(false, " Off ".is_truey());
            assert_eq!(false, "off".is_truey());
            assert_eq!(false, "OFF".is_truey());

            assert_eq!(true, "1".is_truey());
            assert_eq!(true, "true".is_truey());
            assert_eq!(true, "TRUE".is_truey());
            assert_eq!(true, "True".is_truey());
            assert_eq!(true, "tRuE".is_truey());
            assert_eq!(true, "yes".is_truey());
            assert_eq!(true, " YES".is_truey());
            assert_eq!(true, "Yes   ".is_truey());
            assert_eq!(true, "yEs".is_truey());
        }

        // is_truthy
        {
            assert_eq!(None, "".is_truthy());

            assert_eq!(Some(false), "0".is_truthy());
            assert_eq!(Some(false), "false".is_truthy());
            assert_eq!(Some(false), " FALSE".is_truthy());
            assert_eq!(Some(false), "False".is_truthy());
            assert_eq!(Some(false), "FaLSe".is_truthy());
            assert_eq!(Some(false), "no".is_truthy());
            assert_eq!(Some(false), "No ".is_truthy());
            assert_eq!(Some(false), "NO".is_truthy());
            assert_eq!(Some(false), " Off ".is_truthy());
            assert_eq!(Some(false), "off".is_truthy());
            assert_eq!(Some(false), "OFF".is_truthy());

            assert_eq!(Some(true), "1".is_truthy());
            assert_eq!(Some(true), "true".is_truthy());
            assert_eq!(Some(true), "TRUE".is_truthy());
            assert_eq!(Some(true), "True".is_truthy());
            assert_eq!(Some(true), "tRuE".is_truthy());
            assert_eq!(Some(true), "yes".is_truthy());
            assert_eq!(Some(true), " YES".is_truthy());
            assert_eq!(Some(true), "Yes   ".is_truthy());
            assert_eq!(Some(true), "yEs".is_truthy());
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //
