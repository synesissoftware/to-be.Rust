// impls/implement_Truthy_for_AsStr.rs

use crate::truthy::Truthy;

use base_traits::AsStr;

impl<T> Truthy for T
where
    T : AsStr,
{
    fn is_truthy(&self) -> Option<bool> {
        crate::parse::string_is_truthy(self.as_str())
    }
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use crate::truthy::Truthy as _;


    #[test]
    fn TEST_AsStr_Truthy() {
        assert_eq!(Some(true), "yes".is_truthy());
        assert_eq!(Some(false), "no".is_truthy());
        assert_eq!(None, "maybe".is_truthy());
    }
}


// ///////////////////////////// end of file //////////////////////////// //
