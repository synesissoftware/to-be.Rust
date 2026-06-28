// terms.rs

/// Directs custom truthyness behaviour.
#[derive(Clone)]
#[derive(Debug)]
pub enum Terms<'a> {
    /// Use the built-in comparison strings.
    Default,
    /// Use the given `*precise_strings` and, optionally, the given
    /// `*lower_strings` to evaluate the truthyness of a given string.
    Strings {
        /// Precise (trimmed, case-sensitive) strings deemed "falsey".
        falsey_precise_strings :   &'a [&'a str],
        /// Lowercase strings deemed "falsey" after ASCII lower-casing.
        falsey_lowercase_strings : &'a [&'a str],
        /// Precise (trimmed, case-sensitive) strings deemed "truey".
        truey_precise_strings :    &'a [&'a str],
        /// Lowercase strings deemed "truey" after ASCII lower-casing.
        truey_lowercase_strings :  &'a [&'a str],
    },
}


// ///////////////////////////// end of file //////////////////////////// //
