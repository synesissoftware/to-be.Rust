//! Example: classify strings with the [`Truthy`] trait and free functions.

use to_be::{
    stock_term_strings,
    string_is_truthy,
    string_is_truthy_with,
    Terms,
    Truthy as _,
};

fn main() {
    let s1 = "no";
    let s2 = "True";
    let s3 = "orange";

    assert_eq!(Some(false), s1.is_truthy());
    assert_eq!(Some(true), s2.is_truthy());
    assert_eq!(None, s3.is_truthy());

    assert_eq!(Some(false), string_is_truthy("off"));
    assert_eq!(Some(true), string_is_truthy(" YES "));
    assert_eq!(None, string_is_truthy("maybe"));

    let stock = stock_term_strings();
    assert!(matches!(stock, Terms::Strings { .. }));

    if let Terms::Strings {
        falsey_precise_strings,
        falsey_lowercase_strings,
        truey_precise_strings: _,
        truey_lowercase_strings: _,
    } = stock
    {
        let custom = Terms::Strings {
            falsey_precise_strings,
            falsey_lowercase_strings,
            truey_precise_strings : &["Da", "YUP"],
            truey_lowercase_strings : &["da", "yup"],
        };

        assert_eq!(Some(true), string_is_truthy_with("Da", custom.clone()));
        assert_eq!(Some(true), string_is_truthy_with("yup", custom));
    }
}


// ///////////////////////////// end of file //////////////////////////// //
