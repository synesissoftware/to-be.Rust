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


// ///////////////////////////// end of file //////////////////////////// //
