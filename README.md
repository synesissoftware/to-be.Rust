# to-be.Rust <!-- omit in toc -->

Simple Rust library determining the truthyness of strings, that is whether they indicate *truey* or *falsy* values.

[![Crates.io](https://img.shields.io/crates/v/to-be.svg)](https://crates.io/crates/to-be)


## Introduction

**to-be** is a library providing facilities for determine whether the truthyness of strings. It implemented in several languages: **to-be.Rust** is the **Rust** implementation.


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


## Terminology

The term "*truthy*" is an unhelpfully overloaded term in the programming world, insofar as it is used to refer to the notion of "truthyness" - whether something can be _deemed to be_ interpretable as truth - and also the true side of that interpretation. In this library, the former interpretation is used, leaving us with the following terms:

* "*truthy*" - whether something can be can be _deemed to be_ interpretable as having truth;
* "*falsey*" - whether an object can be _deemed to be_ interpretable as being false;
* "*truey*" - whether an object can be _deemed to be_ interpretable as being true;

For example, consider the following **Rust** program:

```Rust
use to_be::Truthy as _;

let s1 = "no";
let s2 = "True";
let s3 = "orange";

// "no" is validly truthy, and is falsey
assert_eq!(true, s1.is_falsey());
assert_eq!(false, s1.is_truey());

// "True" is validly truthy, and is truey
assert_eq!(false, s2.is_falsey());
assert_eq!(true, s2.is_truey());

// "orange" is not validly truthy, and is neither falsey nor truey
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

No public enumerations are defined at this time.


### Features

No public crate-specific features are defined at this time.


### Functions

The following public functions are defined in the current version:

```Rust
pub fn string_is_falsey(s : &str) -> bool;
pub fn string_is_truey(s : &str) -> bool;

pub fn pub fn string_is_truthy(s : &str) -> Option<bool>;
pub fn string_is_truthy_with(
    s : &str,
    terms : Terms,
) -> Option<bool>;

pub fn stock_term_strings() -> Terms<'static>;
```


### Macros

No public macros are defined at this time.


### Structures

The following public structures are defined in the current version:

```Rust
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
        truey_precise_strings :   &'a [&'a str],
        truey_lowercase_strings : &'a [&'a str],
    },
}
```


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

Further, **Truthy** is implemented for any type for which **base-traits**' `AsStr` trait is defined, e.g.

```Rust
use to_be::Truthy as _;

let s : String = "yes".into();

assert!(s.is_truey());
```


## Examples

No example programs are provided at this time.


## Project Information

### Where to get help

[GitHub Page](https://github.com/synesissoftware/to-be.Rust "GitHub Page")


### Contribution guidelines

Defect reports, feature requests, and pull requests are welcome on https://github.com/synesissoftware/to-be.Rust.


### Dependencies

Crates upon which **to-be.Rust** has runtime dependencies:

* [**base-traits**](https://github.com/synesissoftware/base-traits);


#### Dev Dependencies

**to-be.Rust** has no (additional) development dependencies.


### Related projects

* [**to-be**](https://github.com/synesissoftware/to-be) (**C**);
* [**to-be.Python**](https://github.com/synesissoftware/to-be.Python);
* [**to-be.Ruby**](https://github.com/synesissoftware/to-be.Ruby);


### License

**to-be.Rust** is released under the 3-clause BSD license. See [LICENSE](./LICENSE) for details.


<!-- ########################### end of file ########################### -->

