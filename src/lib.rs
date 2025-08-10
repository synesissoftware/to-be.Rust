// lib.rs - to-be API


#[rustfmt::skip]
mod constants {
    #![allow(clippy::redundant_static_lifetimes)]

    // NOTE: `FALSEY_PRECISE_STRINGS` and `TRUEY_PRECISE_STRINGS` must be in sorted order as
    // they are consumed in binary search; the others in most-likely order.

    pub(super) const FALSEY_PRECISE_STRINGS : &'static [&'static str; 10] = &[
        "0",
        "FALSE",
        "False",
        "NO",
        "No",
        "OFF",
        "Off",
        "false",
        "no",
        "off",
    ];

    pub(super) const TRUEY_PRECISE_STRINGS : &'static [&'static str; 10] = &[
        "1",
        "ON",
        "On",
        "TRUE",
        "True",
        "YES",
        "Yes",
        "on",
        "true",
        "yes",
    ];

    pub(super) const FALSEY_LOWERCASE_STRINGS : &'static [&'static str; 4] = &[
        "false",
        "no",
        "off",
        "0",
    ];

    pub(super) const TRUEY_LOWERCASE_STRINGS : &'static [&'static str; 4] = &[
        "true",
        "yes",
        "on",
        "1",
    ];
}


/// Directs custom truthyness behaviour.
#[derive(Clone)]
#[derive(Debug)]
pub enum Terms<'a> {
    /// Use the built-in comparison strings.
    Default,
    /// Use the given `*precise_strings` and, optionally, the given
    /// `*lower_strings` to evaluate the truthyness of a given string.
    Strings {
        falsey_precise_strings :   &'a [&'a str],
        falsey_lowercase_strings : &'a [&'a str],
        truey_precise_strings :    &'a [&'a str],
        truey_lowercase_strings :  &'a [&'a str],
    },
}

fn string_is_truthy_against_(
    s : &str,
    sorted_precise_strings : &[&str],
    lowercase_strings : &[&str],
) -> bool {
    let s = s.trim();

    if sorted_precise_strings.binary_search(&s).is_ok() {
        true
    } else {
        let l = s.to_ascii_lowercase();

        lowercase_strings.iter().any(|&f| f == l)
    }
}

fn string_is_truthy_with_(
    s : &str,
    terms : Terms,
    stock_falsey_sorted_precise_strings : &[&str],
    stock_falsey_lowercase_strings : &[&str],
    stock_truey_sorted_precise_strings : &[&str],
    stock_truey_lowercase_strings : &[&str],
) -> Option<bool> {
    let s = s.trim();

    match terms {
        Terms::Default => {
            if stock_falsey_sorted_precise_strings.binary_search(&s).is_ok() {
                return Some(false);
            }
            if stock_truey_sorted_precise_strings.binary_search(&s).is_ok() {
                return Some(true);
            }
        },
        Terms::Strings {
            falsey_precise_strings,
            truey_precise_strings,
            ..
        } => {
            if falsey_precise_strings.contains(&s) {
                return Some(false);
            }
            if truey_precise_strings.contains(&s) {
                return Some(true);
            }
        },
    };

    let l = s.to_ascii_lowercase();
    let (falsey_lowercase_strings, truey_lowercase_strings) = match terms {
        Terms::Default => (stock_falsey_lowercase_strings, stock_truey_lowercase_strings),
        Terms::Strings {
            falsey_lowercase_strings,
            truey_lowercase_strings,
            ..
        } => (falsey_lowercase_strings, truey_lowercase_strings),
    };

    if falsey_lowercase_strings.iter().any(|&f| f == l) {
        return Some(false);
    }
    if truey_lowercase_strings.iter().any(|&f| f == l) {
        return Some(true);
    }

    None
}

/// Obtain the stock term strings of the library.
///
/// This may be handy when you want to, say, provide your own "truey" term
/// strings but rely on the stock "falsey" term strings.
pub fn stock_term_strings() -> Terms<'static> {
    Terms::Strings {
        falsey_precise_strings :   constants::FALSEY_PRECISE_STRINGS,
        falsey_lowercase_strings : constants::FALSEY_LOWERCASE_STRINGS,
        truey_precise_strings :    constants::TRUEY_PRECISE_STRINGS,
        truey_lowercase_strings :  constants::TRUEY_LOWERCASE_STRINGS,
    }
}

/// Indicates that the given string, when trimmed, is deemed as "truey".
///
/// # Note:
/// It is NOT guaranteed that `string_is_falsey(x) == !string_is_truey(x)`.
pub fn string_is_falsey(s : &str) -> bool {
    string_is_truthy_against_(
        s,
        constants::FALSEY_PRECISE_STRINGS,
        constants::FALSEY_LOWERCASE_STRINGS,
    )
}

/// Indicates that the given string, when trimmed, is deemed as "falsy".
///
/// # Note:
/// It is NOT guaranteed that `string_is_falsey(x) == !string_is_truey(x)`.
///
/// # Returns:
/// - `None` - string is not classified as "truthy";
/// - `Some(false)` - string (is classified as "truthy" and) is deemed
///   "falsey";
/// - `Some(true)` - string (is classified as "truthy" and) is deemed
///   "truey";
pub fn string_is_truey(s : &str) -> bool {
    string_is_truthy_against_(
        s,
        constants::TRUEY_PRECISE_STRINGS,
        constants::TRUEY_LOWERCASE_STRINGS,
    )
}

/// Indicates whether the given string is "truthy" and, if so, whether it is
/// "truey" or "falsey".
///
/// # Returns:
/// - `None` - string is not classified as "truthy";
/// - `Some(false)` - string (is classified as "truthy" and) is deemed
///   "falsey";
/// - `Some(true)` - string (is classified as "truthy" and) is deemed
///   "truey";
pub fn string_is_truthy(s : &str) -> Option<bool> {
    string_is_truthy_with_(
        s,
        Terms::Default,
        constants::FALSEY_PRECISE_STRINGS,
        constants::FALSEY_LOWERCASE_STRINGS,
        constants::TRUEY_PRECISE_STRINGS,
        constants::TRUEY_LOWERCASE_STRINGS,
    )
}

/// Indicates whether the instance can be classed as "truthy" when evaluated
/// against the given terms strings.
pub fn string_is_truthy_with(
    s : &str,
    terms : Terms,
) -> Option<bool> {
    string_is_truthy_with_(
        s,
        terms,
        constants::FALSEY_PRECISE_STRINGS,
        constants::FALSEY_LOWERCASE_STRINGS,
        constants::TRUEY_PRECISE_STRINGS,
        constants::TRUEY_LOWERCASE_STRINGS,
    )
}

/// Trait that provides truthy attributes for an implementing type.
pub trait Truthy {
    /// Indicates whether the instance can be classed as "falsey".
    fn is_falsey(&self) -> bool {
        Some(false) == self.is_truthy()
    }
    /// Indicates whether the instance can be classed as "truey".
    fn is_truey(&self) -> bool {
        Some(true) == self.is_truthy()
    }
    /// Indicates whether the instance can be classed as "truthy", and, if
    /// so, whether it is "truey" or "falsey".
    fn is_truthy(&self) -> Option<bool>;
}

/// Specialisation of [Truthy] for type `T` for any type that implements
/// [AsStr].
#[cfg(feature = "implement-Truthy-for-AsStr")]
#[allow(non_snake_case)]
mod implement_Truthy_for_AsStr {
    use super::Truthy;
    use base_traits::AsStr;

    impl<T> Truthy for T
    where
        T : AsStr,
    {
        fn is_truthy(&self) -> Option<bool> {
            super::string_is_truthy(self.as_str())
        }
    }
}

#[cfg(feature = "implement-Truthy-for-bool")]
#[allow(non_snake_case)]
mod implement_Truthy_for_bool {
    use super::Truthy;

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
            self.map(|&b| b)
        }
    }

    impl Truthy for &Option<&bool> {
        fn is_truthy(&self) -> Option<bool> {
            self.map(|&b| b)
        }
    }
}

#[cfg(feature = "implement-Truthy-for-str")]
#[allow(non_snake_case)]
mod implement_Truthy_for_str {
    use super::Truthy;

    impl Truthy for &str {
        fn is_truthy(&self) -> Option<bool> {
            super::string_is_truthy(self)
        }
    }

    impl Truthy for &&str {
        fn is_truthy(&self) -> Option<bool> {
            super::string_is_truthy(*self)
        }
    }
}

#[cfg(feature = "implement-Truthy-for-String")]
#[allow(non_snake_case)]
mod implement_Truthy_for_String {
    use super::Truthy;

    impl Truthy for String {
        fn is_truthy(&self) -> Option<bool> {
            super::string_is_truthy(self.as_str())
        }
    }

    impl Truthy for &String {
        fn is_truthy(&self) -> Option<bool> {
            super::string_is_truthy(self.as_str())
        }
    }
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]


    mod test_API {
        #![allow(non_snake_case)]

        use super::super::{
            string_is_falsey,
            string_is_truey,
            string_is_truthy,
            string_is_truthy_with,
            Terms,
        };


        #[test]
        fn TEST_string_is_falsey_1() {
            assert_eq!(false, string_is_falsey(""));

            assert_eq!(true, string_is_falsey("0"));
            assert_eq!(true, string_is_falsey("false"));
            assert_eq!(true, string_is_falsey(" FALSE"));
            assert_eq!(true, string_is_falsey("False"));
            assert_eq!(true, string_is_falsey("FaLSe"));
            assert_eq!(true, string_is_falsey("no"));
            assert_eq!(true, string_is_falsey("No "));
            assert_eq!(true, string_is_falsey("NO"));
            assert_eq!(true, string_is_falsey(" Off "));
            assert_eq!(true, string_is_falsey("off"));
            assert_eq!(true, string_is_falsey("OFF"));

            assert_eq!(false, string_is_falsey("1"));
            assert_eq!(false, string_is_falsey("true"));
            assert_eq!(false, string_is_falsey("TRUE"));
            assert_eq!(false, string_is_falsey("True"));
            assert_eq!(false, string_is_falsey("tRuE"));
            assert_eq!(false, string_is_falsey("yes"));
            assert_eq!(false, string_is_falsey(" YES"));
            assert_eq!(false, string_is_falsey("Yes   "));
            assert_eq!(false, string_is_falsey("yEs"));
        }

        #[test]
        fn TEST_string_is_truey_1() {
            assert_eq!(false, string_is_truey(""));

            assert_eq!(false, string_is_truey("0"));
            assert_eq!(false, string_is_truey("false"));
            assert_eq!(false, string_is_truey(" FALSE"));
            assert_eq!(false, string_is_truey("False"));
            assert_eq!(false, string_is_truey("FaLSe"));
            assert_eq!(false, string_is_truey("no"));
            assert_eq!(false, string_is_truey("No "));
            assert_eq!(false, string_is_truey("NO"));
            assert_eq!(false, string_is_truey(" Off "));
            assert_eq!(false, string_is_truey("off"));
            assert_eq!(false, string_is_truey("OFF"));

            assert_eq!(true, string_is_truey("1"));
            assert_eq!(true, string_is_truey("true"));
            assert_eq!(true, string_is_truey("TRUE"));
            assert_eq!(true, string_is_truey("True"));
            assert_eq!(true, string_is_truey("tRuE"));
            assert_eq!(true, string_is_truey("yes"));
            assert_eq!(true, string_is_truey(" YES"));
            assert_eq!(true, string_is_truey("Yes   "));
            assert_eq!(true, string_is_truey("yEs"));
        }

        #[test]
        fn TEST_string_is_truthy_1() {
            assert_eq!(None, string_is_truthy(""));

            assert_eq!(None, string_is_truthy("Nyet"));
            assert_eq!(None, string_is_truthy("NYET"));
            assert_eq!(None, string_is_truthy("nyET"));
            assert_eq!(None, string_is_truthy("nope"));
            assert_eq!(None, string_is_truthy("Nope"));
            assert_eq!(None, string_is_truthy("NOPE"));

            assert_eq!(None, string_is_truthy("Da"));
            assert_eq!(None, string_is_truthy("DA"));
            assert_eq!(None, string_is_truthy("dA"));
            assert_eq!(None, string_is_truthy("yup"));
            assert_eq!(None, string_is_truthy("Yup"));
            assert_eq!(None, string_is_truthy("yUp"));

            assert_eq!(Some(false), string_is_truthy("0"));
            assert_eq!(Some(false), string_is_truthy("false"));
            assert_eq!(Some(false), string_is_truthy(" FALSE"));
            assert_eq!(Some(false), string_is_truthy("False"));
            assert_eq!(Some(false), string_is_truthy("FaLSe"));
            assert_eq!(Some(false), string_is_truthy("no"));
            assert_eq!(Some(false), string_is_truthy("No "));
            assert_eq!(Some(false), string_is_truthy("NO"));
            assert_eq!(Some(false), string_is_truthy(" Off "));
            assert_eq!(Some(false), string_is_truthy("off"));
            assert_eq!(Some(false), string_is_truthy("OFF"));

            assert_eq!(Some(true), string_is_truthy("1"));
            assert_eq!(Some(true), string_is_truthy("true"));
            assert_eq!(Some(true), string_is_truthy("TRUE"));
            assert_eq!(Some(true), string_is_truthy("True"));
            assert_eq!(Some(true), string_is_truthy("tRuE"));
            assert_eq!(Some(true), string_is_truthy("yes"));
            assert_eq!(Some(true), string_is_truthy(" YES"));
            assert_eq!(Some(true), string_is_truthy("Yes   "));
            assert_eq!(Some(true), string_is_truthy("yEs"));
        }

        #[test]
        fn TEST_string_is_truthy_with_1() {
            #[rustfmt::skip]
            const TRUEY_PRECISE_STRINGS : &[&str] = &[
                "Da",
                "YUP",
                "Yup",
            ];
            #[rustfmt::skip]
            const TRUEY_LOWERCASE_STRINGS : &[&str] = &[
                "da",
                "yup",
            ];
            #[rustfmt::skip]
            const FALSEY_PRECISE_STRINGS : &[&str] = &[
                "Nyet",
                "Nope",
            ];
            #[rustfmt::skip]
            const FALSEY_LOWERCASE_STRINGS : &[&str] = &[
                "nyet",
                "nope",
            ];

            let terms = Terms::Strings {
                falsey_precise_strings :   FALSEY_PRECISE_STRINGS,
                falsey_lowercase_strings : FALSEY_LOWERCASE_STRINGS,
                truey_precise_strings :    TRUEY_PRECISE_STRINGS,
                truey_lowercase_strings :  TRUEY_LOWERCASE_STRINGS,
            };

            assert_eq!(Some(false), string_is_truthy_with("Nyet", terms.clone()));
            assert_eq!(Some(false), string_is_truthy_with("NYET", terms.clone()));
            assert_eq!(Some(false), string_is_truthy_with("nyET", terms.clone()));
            assert_eq!(Some(false), string_is_truthy_with("nope", terms.clone()));
            assert_eq!(Some(false), string_is_truthy_with("Nope", terms.clone()));
            assert_eq!(Some(false), string_is_truthy_with("NOPE", terms.clone()));

            assert_eq!(Some(true), string_is_truthy_with("Da", terms.clone()));
            assert_eq!(Some(true), string_is_truthy_with("DA", terms.clone()));
            assert_eq!(Some(true), string_is_truthy_with("dA", terms.clone()));
            assert_eq!(Some(true), string_is_truthy_with("yup", terms.clone()));
            assert_eq!(Some(true), string_is_truthy_with("Yup", terms.clone()));
            assert_eq!(Some(true), string_is_truthy_with("yUp", terms.clone()));

            assert_eq!(None, string_is_truthy_with("", terms.clone()));

            assert_eq!(None, string_is_truthy_with("0", terms.clone()));
            assert_eq!(None, string_is_truthy_with("false", terms.clone()));
            assert_eq!(None, string_is_truthy_with(" FALSE", terms.clone()));
            assert_eq!(None, string_is_truthy_with("False", terms.clone()));
            assert_eq!(None, string_is_truthy_with("FaLSe", terms.clone()));
            assert_eq!(None, string_is_truthy_with("no", terms.clone()));
            assert_eq!(None, string_is_truthy_with("No ", terms.clone()));
            assert_eq!(None, string_is_truthy_with("NO", terms.clone()));
            assert_eq!(None, string_is_truthy_with(" Off ", terms.clone()));
            assert_eq!(None, string_is_truthy_with("off", terms.clone()));
            assert_eq!(None, string_is_truthy_with("OFF", terms.clone()));

            assert_eq!(None, string_is_truthy_with("1", terms.clone()));
            assert_eq!(None, string_is_truthy_with("true", terms.clone()));
            assert_eq!(None, string_is_truthy_with("TRUE", terms.clone()));
            assert_eq!(None, string_is_truthy_with("True", terms.clone()));
            assert_eq!(None, string_is_truthy_with("tRuE", terms.clone()));
            assert_eq!(None, string_is_truthy_with("yes", terms.clone()));
            assert_eq!(None, string_is_truthy_with(" YES", terms.clone()));
            assert_eq!(None, string_is_truthy_with("Yes   ", terms.clone()));
            assert_eq!(None, string_is_truthy_with("yEs", terms.clone()));
        }
    }

    mod test_Truthy {
        #![allow(non_snake_case)]

        #[cfg(any(
            feature = "implement-Truthy-for-AsStr",
            feature = "implement-Truthy-for-String",
            feature = "implement-Truthy-for-bool",
            feature = "implement-Truthy-for-str",
        ))]
        use super::super::Truthy as _;


        #[cfg(feature = "implement-Truthy-for-String")]
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

        #[cfg(feature = "implement-Truthy-for-bool")]
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

        #[cfg(any(
            feature = "implement-Truthy-for-AsStr",
            feature = "implement-Truthy-for-str",
        ))]
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
}
