// constants.rs

/// Stock string tables used by the default truthy evaluation paths.
///
/// # Note:
/// `FALSEY_PRECISE_STRINGS` and `TRUEY_PRECISE_STRINGS` must remain in
/// sorted order because they are queried via binary search. The lowercase
/// tables are scanned linearly after ASCII lower-casing.
#[rustfmt::skip]
pub(crate) mod stock {
    #![allow(clippy::redundant_static_lifetimes)]

    pub(crate) const FALSEY_PRECISE_STRINGS : &'static [&'static str; 10] = &[
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

    pub(crate) const TRUEY_PRECISE_STRINGS : &'static [&'static str; 10] = &[
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

    pub(crate) const FALSEY_LOWERCASE_STRINGS : &'static [&'static str; 4] = &[
        "false",
        "no",
        "off",
        "0",
    ];

    pub(crate) const TRUEY_LOWERCASE_STRINGS : &'static [&'static str; 4] = &[
        "true",
        "yes",
        "on",
        "1",
    ];
}


// ///////////////////////////// end of file //////////////////////////// //
