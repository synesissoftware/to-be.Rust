// parse.rs

use crate::{
    constants::stock,
    Terms,
};

fn conditional_trim_(
    s : &str
) -> (
    &str, // s
    bool, // was_trimmed
) {
    let l = s.len();

    if l > 1 {
        let b_0 = s.as_bytes()[0];

        if b_0.is_ascii_whitespace() {
            return (s.trim(), true);
        }

        let b_l = s.as_bytes()[l - 1];

        if b_l.is_ascii_whitespace() {
            return (s.trim(), true);
        }
    }

    (s, false)
}

fn string_is_truthy_against_(
    s : &str,
    sorted_precise_strings : &[&str],
    lowercase_strings : &[&str],
) -> bool {
    let (s, was_trimmed) = conditional_trim_(s);

    if sorted_precise_strings.binary_search(&s).is_ok() {
        true
    } else {
        let s = if was_trimmed { s } else { s.trim() };
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
    let (s, was_trimmed) = conditional_trim_(s);

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

    let s = if was_trimmed { s } else { s.trim() };
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
        falsey_precise_strings :   stock::FALSEY_PRECISE_STRINGS,
        falsey_lowercase_strings : stock::FALSEY_LOWERCASE_STRINGS,
        truey_precise_strings :    stock::TRUEY_PRECISE_STRINGS,
        truey_lowercase_strings :  stock::TRUEY_LOWERCASE_STRINGS,
    }
}

/// Indicates that the given string, when trimmed, is deemed as "falsey".
///
/// # Note:
/// It is NOT guaranteed that `string_is_falsey(x) == !string_is_truey(x)`.
pub fn string_is_falsey(s : &str) -> bool {
    let sorted_precise_strings = stock::FALSEY_PRECISE_STRINGS;
    let lowercase_strings = stock::FALSEY_LOWERCASE_STRINGS;

    string_is_truthy_against_(s, sorted_precise_strings, lowercase_strings)
}

/// Indicates that the given string, when trimmed, is deemed as "truey".
///
/// # Note:
/// It is NOT guaranteed that `string_is_falsey(x) == !string_is_truey(x)`.
pub fn string_is_truey(s : &str) -> bool {
    let sorted_precise_strings = stock::TRUEY_PRECISE_STRINGS;
    let lowercase_strings = stock::TRUEY_LOWERCASE_STRINGS;

    string_is_truthy_against_(s, sorted_precise_strings, lowercase_strings)
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
        stock::FALSEY_PRECISE_STRINGS,
        stock::FALSEY_LOWERCASE_STRINGS,
        stock::TRUEY_PRECISE_STRINGS,
        stock::TRUEY_LOWERCASE_STRINGS,
    )
}

/// Indicates whether the given string is "truthy" when evaluated against
/// the given term strings.
///
/// # Returns:
/// - `None` - string is not classified as "truthy";
/// - `Some(false)` - string (is classified as "truthy" and) is deemed
///   "falsey";
/// - `Some(true)` - string (is classified as "truthy" and) is deemed
///   "truey";
pub fn string_is_truthy_with(
    s : &str,
    terms : Terms,
) -> Option<bool> {
    string_is_truthy_with_(
        s,
        terms,
        stock::FALSEY_PRECISE_STRINGS,
        stock::FALSEY_LOWERCASE_STRINGS,
        stock::TRUEY_PRECISE_STRINGS,
        stock::TRUEY_LOWERCASE_STRINGS,
    )
}

/// Indicates whether the given OS string is "truthy" and, if so, whether it
/// is "truey" or "falsey".
///
/// Non-UTF-8 OS strings are not classified and yield `None`.
///
/// # Returns:
/// - `None` - string is not classified as "truthy", or is not valid UTF-8;
/// - `Some(false)` - string (is classified as "truthy" and) is deemed
///   "falsey";
/// - `Some(true)` - string (is classified as "truthy" and) is deemed
///   "truey".
pub fn os_string_is_truthy(s : &std::ffi::OsStr) -> Option<bool> {
    match s.to_str() {
        None => None,
        Some(c) => string_is_truthy(c),
    }
}


#[cfg(test)]
mod tests {
    #![allow(clippy::bool_assert_comparison)]
    #![allow(non_snake_case)]

    use super::{
        stock_term_strings,
        string_is_falsey,
        string_is_truey,
        string_is_truthy,
        string_is_truthy_with,
    };
    use crate::{
        constants::stock,
        Terms,
    };


    #[test]
    fn TEST_stock_tables_are_sorted() {
        assert!(stock::FALSEY_PRECISE_STRINGS.windows(2).all(|w| w[0] < w[1]));
        assert!(stock::TRUEY_PRECISE_STRINGS.windows(2).all(|w| w[0] < w[1]));
    }

    #[test]
    fn TEST_stock_term_strings_returns_stock_tables() {
        let terms = stock_term_strings();
        match terms {
            Terms::Strings {
                falsey_precise_strings,
                falsey_lowercase_strings,
                truey_precise_strings,
                truey_lowercase_strings,
            } => {
                assert_eq!(stock::FALSEY_PRECISE_STRINGS, falsey_precise_strings);
                assert_eq!(stock::FALSEY_LOWERCASE_STRINGS, falsey_lowercase_strings);
                assert_eq!(stock::TRUEY_PRECISE_STRINGS, truey_precise_strings);
                assert_eq!(stock::TRUEY_LOWERCASE_STRINGS, truey_lowercase_strings);
            },
            Terms::Default => panic!("expected Terms::Strings"),
        }
    }

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
    fn TEST_string_is_truthy_WHITESPACE_PADDED_1() {
        macro_rules! assert_stock_truthy {
            ($s:expr, $truthy:expr, $truey:expr, $falsey:expr) => {
                assert_eq!($truthy, string_is_truthy($s), "string_is_truthy({:?})", $s);
                assert_eq!($truey, string_is_truey($s), "string_is_truey({:?})", $s);
                assert_eq!($falsey, string_is_falsey($s), "string_is_falsey({:?})", $s);
            };
        }

        // Leading ASCII space
        assert_stock_truthy!(" true", Some(true), true, false);
        assert_stock_truthy!(" TRUE", Some(true), true, false);
        assert_stock_truthy!(" True", Some(true), true, false);
        assert_stock_truthy!(" false", Some(false), false, true);
        assert_stock_truthy!(" FALSE", Some(false), false, true);
        assert_stock_truthy!(" False", Some(false), false, true);
        assert_stock_truthy!(" yes", Some(true), true, false);
        assert_stock_truthy!(" YES", Some(true), true, false);
        assert_stock_truthy!(" Yes", Some(true), true, false);
        assert_stock_truthy!(" no", Some(false), false, true);
        assert_stock_truthy!(" NO", Some(false), false, true);
        assert_stock_truthy!(" No", Some(false), false, true);
        assert_stock_truthy!(" on", Some(true), true, false);
        assert_stock_truthy!(" ON", Some(true), true, false);
        assert_stock_truthy!(" On", Some(true), true, false);
        assert_stock_truthy!(" off", Some(false), false, true);
        assert_stock_truthy!(" OFF", Some(false), false, true);
        assert_stock_truthy!(" Off", Some(false), false, true);
        assert_stock_truthy!(" 1", Some(true), true, false);
        assert_stock_truthy!(" 0", Some(false), false, true);

        // Trailing ASCII space
        assert_stock_truthy!("true ", Some(true), true, false);
        assert_stock_truthy!("TRUE ", Some(true), true, false);
        assert_stock_truthy!("True ", Some(true), true, false);
        assert_stock_truthy!("false ", Some(false), false, true);
        assert_stock_truthy!("FALSE ", Some(false), false, true);
        assert_stock_truthy!("False ", Some(false), false, true);
        assert_stock_truthy!("yes ", Some(true), true, false);
        assert_stock_truthy!("YES ", Some(true), true, false);
        assert_stock_truthy!("Yes ", Some(true), true, false);
        assert_stock_truthy!("no ", Some(false), false, true);
        assert_stock_truthy!("NO ", Some(false), false, true);
        assert_stock_truthy!("No ", Some(false), false, true);
        assert_stock_truthy!("on ", Some(true), true, false);
        assert_stock_truthy!("ON ", Some(true), true, false);
        assert_stock_truthy!("On ", Some(true), true, false);
        assert_stock_truthy!("off ", Some(false), false, true);
        assert_stock_truthy!("OFF ", Some(false), false, true);
        assert_stock_truthy!("Off ", Some(false), false, true);
        assert_stock_truthy!("1 ", Some(true), true, false);
        assert_stock_truthy!("0 ", Some(false), false, true);

        // Leading and trailing ASCII space
        assert_stock_truthy!(" true ", Some(true), true, false);
        assert_stock_truthy!(" TRUE ", Some(true), true, false);
        assert_stock_truthy!(" True ", Some(true), true, false);
        assert_stock_truthy!(" false ", Some(false), false, true);
        assert_stock_truthy!(" FALSE ", Some(false), false, true);
        assert_stock_truthy!(" False ", Some(false), false, true);
        assert_stock_truthy!(" yes ", Some(true), true, false);
        assert_stock_truthy!(" YES ", Some(true), true, false);
        assert_stock_truthy!(" Yes ", Some(true), true, false);
        assert_stock_truthy!(" no ", Some(false), false, true);
        assert_stock_truthy!(" NO ", Some(false), false, true);
        assert_stock_truthy!(" No ", Some(false), false, true);
        assert_stock_truthy!(" on ", Some(true), true, false);
        assert_stock_truthy!(" ON ", Some(true), true, false);
        assert_stock_truthy!(" On ", Some(true), true, false);
        assert_stock_truthy!(" off ", Some(false), false, true);
        assert_stock_truthy!(" OFF ", Some(false), false, true);
        assert_stock_truthy!(" Off ", Some(false), false, true);
        assert_stock_truthy!(" 1 ", Some(true), true, false);
        assert_stock_truthy!(" 0 ", Some(false), false, true);

        // Tab padding (trimmed before classification)
        assert_stock_truthy!("\ttrue\t", Some(true), true, false);
        assert_stock_truthy!("\tFALSE\t", Some(false), false, true);
        assert_stock_truthy!("\t yes \t", Some(true), true, false);

        // Mixed case with padding (lowercase-table path after trim)
        assert_stock_truthy!(" tRuE ", Some(true), true, false);
        assert_stock_truthy!(" FaLSe ", Some(false), false, true);
        assert_stock_truthy!(" yEs ", Some(true), true, false);

        // Unrecognised and whitespace-only inputs
        assert_stock_truthy!(" unrecognised ", None, false, false);
        assert_stock_truthy!("   ", None, false, false);
        assert_stock_truthy!("\t", None, false, false);
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

        assert_eq!(Some(false), string_is_truthy_with(" Nyet ", terms.clone()));
        assert_eq!(Some(false), string_is_truthy_with(" NYET ", terms.clone()));
        assert_eq!(Some(false), string_is_truthy_with(" nyET ", terms.clone()));
        assert_eq!(Some(false), string_is_truthy_with(" nope ", terms.clone()));
        assert_eq!(Some(false), string_is_truthy_with(" Nope ", terms.clone()));
        assert_eq!(Some(false), string_is_truthy_with(" NOPE ", terms.clone()));

        assert_eq!(Some(true), string_is_truthy_with(" Da ", terms.clone()));
        assert_eq!(Some(true), string_is_truthy_with(" DA ", terms.clone()));
        assert_eq!(Some(true), string_is_truthy_with(" dA ", terms.clone()));
        assert_eq!(Some(true), string_is_truthy_with(" yup ", terms.clone()));
        assert_eq!(Some(true), string_is_truthy_with(" Yup ", terms.clone()));
        assert_eq!(Some(true), string_is_truthy_with(" yUp ", terms.clone()));

        assert_eq!(None, string_is_truthy_with("   ", terms.clone()));
    }
}


// ///////////////////////////// end of file //////////////////////////// //
