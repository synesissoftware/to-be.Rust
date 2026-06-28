// impls/implement_Truthy_for_bool.rs

use crate::truthy::Truthy;

impl Truthy for bool {
    fn is_truthy(&self) -> Option<bool> {
        Some(*self)
    }
}

impl Truthy for &bool {
    fn is_truthy(&self) -> Option<bool> {
        Some(**self)
    }
}

impl Truthy for Option<bool> {
    fn is_truthy(&self) -> Option<bool> {
        *self
    }
}

impl Truthy for &Option<bool> {
    fn is_truthy(&self) -> Option<bool> {
        **self
    }
}

impl Truthy for Option<&bool> {
    fn is_truthy(&self) -> Option<bool> {
        self.copied()
    }
}

impl Truthy for &Option<&bool> {
    fn is_truthy(&self) -> Option<bool> {
        (*self).copied()
    }
}


#[cfg(test)]
mod tests {
    #![allow(clippy::bool_assert_comparison)]
    #![allow(clippy::needless_borrow)]
    #![allow(non_snake_case)]

    use crate::truthy::Truthy as _;


    #[test]
    fn TEST_bool_Truthy() {
        // bool
        {
            assert_eq!(true, false.is_falsey());
            assert_eq!(false, false.is_truey());
            assert_eq!(Some(false), false.is_truthy());

            assert_eq!(false, true.is_falsey());
            assert_eq!(true, true.is_truey());
            assert_eq!(Some(true), true.is_truthy());
        }

        // &bool
        {
            assert_eq!(true, (&false).is_falsey());
            assert_eq!(false, (&false).is_truey());
            assert_eq!(Some(false), (&false).is_truthy());

            assert_eq!(false, (&true).is_falsey());
            assert_eq!(true, (&true).is_truey());
            assert_eq!(Some(true), (&true).is_truthy());
        }

        // Option<bool>
        {
            assert_eq!(true, Some(false).is_falsey());
            assert_eq!(false, Some(false).is_truey());
            assert_eq!(Some(false), Some(false).is_truthy());

            assert_eq!(false, Some(true).is_falsey());
            assert_eq!(true, Some(true).is_truey());
            assert_eq!(Some(true), Some(true).is_truthy());
        }

        // Option<&bool>
        {
            assert_eq!(true, Some(&false).is_falsey());
            assert_eq!(false, Some(&false).is_truey());
            assert_eq!(Some(false), Some(&false).is_truthy());

            assert_eq!(false, Some(&true).is_falsey());
            assert_eq!(true, Some(&true).is_truey());
            assert_eq!(Some(true), Some(&true).is_truthy());
        }

        // &Option<bool>
        {
            assert_eq!(true, (&Some(false)).is_falsey());
            assert_eq!(false, (&Some(false)).is_truey());
            assert_eq!(Some(false), (&Some(false)).is_truthy());

            assert_eq!(false, (&Some(true)).is_falsey());
            assert_eq!(true, (&Some(true)).is_truey());
            assert_eq!(Some(true), (&Some(true)).is_truthy());
        }

        // &Option<&bool>
        {
            assert_eq!(true, (&Some(&false)).is_falsey());
            assert_eq!(false, (&Some(&false)).is_truey());
            assert_eq!(Some(false), (&Some(&false)).is_truthy());

            assert_eq!(false, (&Some(&true)).is_falsey());
            assert_eq!(true, (&Some(&true)).is_truey());
            assert_eq!(Some(true), (&Some(&true)).is_truthy());
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //
