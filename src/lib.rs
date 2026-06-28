//! Simple truthy/falsy string evaluation for Rust — part of the
//! cross-language **to-be** family.
//!
//! **to-be** classifies strings commonly found in configuration and
//! environment variables — such as `"yes"`, `"no"`, `"true"`, and
//! `"off"` — as *truthy* (recognised), *truey* (interpreted as true),
//! or *falsey* (interpreted as false). Strings that are not recognised
//! yield `None` from [`string_is_truthy`] and the [`Truthy::is_truthy`]
//! trait method.
//!
//! The library is implemented in several languages; **to-be.Rust** is the
//! Rust port.
//!
//! # Terminology
//!
//! * *truthy* — the string can be interpreted as having a boolean meaning;
//! * *truey* — the string is interpreted as true;
//! * *falsey* — the string is interpreted as false.
//!
//! Note that [`string_is_falsey`] and [`string_is_truey`] answer narrower
//! questions than [`string_is_truthy`]: an empty string is neither truey
//! nor falsey, so both helper functions return `false`.
//!
//! # Installation
//!
//! Reference in **Cargo.toml** in the usual way:
//!
//! ```toml
//! to-be = { version = "0" }
//! ```
//!
//! # Components
//!
//! ## Functions
//!
//! * [`string_is_falsey`] — is the trimmed string a recognised falsey term;
//! * [`string_is_truey`] — is the trimmed string a recognised truey term;
//! * [`string_is_truthy`] — full three-way classification using
//!   stock terms;
//! * [`string_is_truthy_with`] — classification against custom [`Terms`];
//! * [`stock_term_strings`] — obtain the built-in term tables as [`Terms`];
//!
//! ## Types
//!
//! * [`Terms`] — selects stock or custom comparison strings;
//! * [`Truthy`] — trait providing `is_truthy()`, `is_truey()`, and
//!   `is_falsey()` on supported types;
//!
//! # Examples
//!
//! ```
//! use to_be::Truthy as _;
//!
//! assert_eq!(Some(false), "no".is_truthy());
//! assert_eq!(Some(true), "True".is_truthy());
//! assert_eq!(None, "orange".is_truthy());
//! ```
//!
//! Further examples are provided in the repository **examples** directory
//! and in the project
//! [README](https://github.com/synesissoftware/to-be.Rust).

// lib.rs

mod constants;
mod impls;
mod parse;
mod terms;
mod truthy;

pub use parse::{
    os_string_is_truthy,
    stock_term_strings,
    string_is_falsey,
    string_is_truey,
    string_is_truthy,
    string_is_truthy_with,
};
pub use terms::Terms;
pub use truthy::Truthy;


// ///////////////////////////// end of file //////////////////////////// //
