# to-be.Rust <!-- omit in toc -->

![Language](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![GitHub release](https://img.shields.io/github/v/release/synesissoftware/to-be.Rust.svg)](https://github.com/synesissoftware/to-be.Rust/releases/latest)
[![Last Commit](https://img.shields.io/github/last-commit/synesissoftware/to-be.Rust)](https://github.com/synesissoftware/to-be.Rust/commits/master)
[![Crates.io](https://img.shields.io/crates/v/to-be.svg)](https://crates.io/crates/to-be)
![MSRV](https://img.shields.io/badge/MSRV-1.74-lightgrey)
[![CI](https://github.com/synesissoftware/to-be.Rust/actions/workflows/ci.yml/badge.svg)](https://github.com/synesissoftware/to-be.Rust/actions/workflows/ci.yml)
[![docs.rs](https://docs.rs/to-be/badge.svg)](https://docs.rs/to-be)

Simple Rust library determining the truthyness of strings, that is whether they indicate *truey* or *falsey* values.


## Table of Contents <!-- omit in toc -->

- [Introduction](#introduction)
- [Terminology](#terminology)
- [Installation](#installation)
- [Components](#components)
  - [Constants](#constants)
  - [Enumerations](#enumerations)
  - [Features](#features)
  - [Functions](#functions)
  - [Macros](#macros)
  - [Structures](#structures)
  - [Traits](#traits)
- [Examples](#examples)
- [Project Information](#project-information)
  - [Where to get help](#where-to-get-help)
  - [Contribution guidelines](#contribution-guidelines)
  - [Dependencies](#dependencies)
    - [Dev Dependencies](#dev-dependencies)
  - [Related projects](#related-projects)
  - [License](#license)


## Introduction

**to-be** is a library providing facilities for determining whether the truthyness of strings. It is implemented in several languages: **to-be.Rust** is the **Rust** implementation.


## Terminology

The term "*truthy*" is an unhelpfully overloaded term in the programming world, insofar as it is used to refer to the notion of "truthyness" - whether something can be _deemed to be_ interpretable as truth - and also the true side of that interpretation. In this library, the former interpretation is used, leaving us with the following terms:

* "*truthy*" - whether something can be _deemed to be_ interpretable as having truth;
* "*falsey*" - whether an object can be _deemed to be_ interpretable as being false;
* "*truey*" - whether an object can be _deemed to be_ interpretable as being true;

For example, consider the following **Rust** program:

```Rust
use to_be::Truthy as _;

let s1 = "no";
let s2 = "True";
let s3 = "orange";

// "no" is validly truthy, and is falsey
assert_eq!(Some(false), s1.is_truthy());
assert_eq!(true, s1.is_falsey());
assert_eq!(false, s1.is_truey());

// "True" is validly truthy, and is truey
assert_eq!(Some(true), s2.is_truthy());
assert_eq!(false, s2.is_falsey());
assert_eq!(true, s2.is_truey());

// "orange" is not validly truthy, and is neither falsey nor truey
assert_eq!(None, s3.is_truthy());
assert_eq!(false, s3.is_falsey());
assert_eq!(false, s3.is_truey());
```


## Installation

Reference in **Cargo.toml** in the usual way:

```toml
to-be = { version = "0" }
```


## Components

### Constants

No public constants are defined at this time.


### Enumerations

The following public enumeration is defined in the current version:

```Rust
#[derive(Clone)]
#[derive(Debug)]
pub enum Terms<'a> {
    /// Use the built-in comparison strings.
    Default,
    /// Use custom precise and lowercase comparison strings.
    Strings {
        falsey_precise_strings :   &'a [&'a str],
        falsey_lowercase_strings : &'a [&'a str],
        truey_precise_strings :    &'a [&'a str],
        truey_lowercase_strings :  &'a [&'a str],
    },
}
```


### Features

The following optional **Cargo.toml** features are defined:

* `null-feature` - a feature that has no effect (and, thus, is useful for simplifying driver scripts);
* `implement-Truthy-for-AsStr` - implements [`Truthy`] for all types that implement `AsStr` from [**base-traits**](https://github.com/synesissoftware/base-traits). **NOTE:** this is incompatible with the other `implement-Truthy-for-*` features;
* `implement-Truthy-for-bool` - implements [`Truthy`] for `bool` (enabled by default);
* `implement-Truthy-for-CStr` - implements [`Truthy`] for `CStr`;
* `implement-Truthy-for-CString` - implements [`Truthy`] for `CString`;
* `implement-Truthy-for-OsStr` - implements [`Truthy`] for `OsStr`;
* `implement-Truthy-for-OsString` - implements [`Truthy`] for `OsString`;
* `implement-Truthy-for-str` - implements [`Truthy`] for `&str` (enabled by default);
* `implement-Truthy-for-String` - implements [`Truthy`] for `String` (enabled by default);


### Functions

The following public functions are defined in the current version:

```Rust
/// Indicates that the given string, when trimmed, is deemed as "falsey".
pub fn string_is_falsey(s : &str) -> bool;
/// Indicates that the given string, when trimmed, is deemed as "truey".
pub fn string_is_truey(s : &str) -> bool;

/// Indicates whether the given string is "truthy" and, if so, whether it is
/// "truey" or "falsey".
pub fn string_is_truthy(s : &str) -> Option<bool>;
/// Indicates whether the given string is "truthy" when evaluated against
/// the given term strings.
pub fn string_is_truthy_with(
    s : &str,
    terms : Terms,
) -> Option<bool>;

/// Obtain the stock term strings of the library.
pub fn stock_term_strings() -> Terms<'static>;

/// Indicates whether the given OS string is "truthy" and, if so, whether it
/// is "truey" or "falsey".
pub fn os_string_is_truthy(s : &std::ffi::OsStr) -> Option<bool>;
```


### Macros

No public macros are defined at this time.


### Structures

No public structures are defined at this time.


### Traits

The following public traits are defined in the current version:

```Rust
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
```

With the default features, **Truthy** is implemented for `bool`, `&str`, and `String`. Enable `implement-Truthy-for-AsStr` to implement **Truthy** for all types that implement **base-traits**' `AsStr` trait instead.


## Examples

An example program is provided in [**examples/truthy_strings.rs**](./examples/truthy_strings.rs):

```sh
cargo run --example truthy_strings
```

### Benchmarks

Run the Criterion benchmarks:

```sh
cargo bench
```

Criterion writes a grouped HTML comparison report (requires the default
**criterion** features, including **plotters**):

```sh
open target/criterion/report/index.html
```

Individual benchmark pages also appear under `target/criterion/<group>/`.


## Project Information

### Where to get help

[GitHub Page](https://github.com/synesissoftware/to-be.Rust "GitHub Page")


### Contribution guidelines

Defect reports, feature requests, and pull requests are welcome on https://github.com/synesissoftware/to-be.Rust.


### Dependencies

Crates upon which **to-be.Rust** has runtime dependencies:

* [**base-traits**](https://github.com/synesissoftware/base-traits);


#### Dev Dependencies

* [**criterion**](https://github.com/bheisler/criterion.rs) (benchmarking only);


### Related projects

* [**to-be**](https://github.com/synesissoftware/to-be) (**C**);
* [**to-be.Python**](https://github.com/synesissoftware/to-be.Python);
* [**to_be.Ruby**](https://github.com/synesissoftware/to_be.Ruby);


### License

**to-be.Rust** is released under the 3-clause BSD license. See [LICENSE](./LICENSE) for details.


<!-- ########################### end of file ########################### -->
