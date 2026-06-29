# **to-be.Rust** Changes

## 0.0.8 - 29th June 2026

* optimised string truthiness parsing with conditional trim (leading/trailing ASCII whitespace lookahead before `trim()` on the precise-match fast path);
* expanded unit tests for whitespace-padded stock terms (including single-digit `" 1"`, `"1 "`, and tab padding);
* extended **benches/string_truthy** Criterion benchmarks (custom `Terms`, padded stock inputs);


## 0.0.7 - 27th June 2025

* upgraded CI (clippy, rustfmt, MSRV job, optional-feature test matrix);
* added crate-level rustdoc;
* aligned **rustfmt.toml** with **Diagnosticism.Rust** / SIS conventions;
* added **benches/string_truthy** Criterion benchmark;
* added **examples/truthy_strings** example program;
* modularised implementation (`constants`, `parse`, `terms`, `truthy`, `impls`);
* added `os_string_is_truthy()` (fixes missing symbol for `OsStr` / `OsString` features);
* expanded tests (stock tables, `CStr`, non-UTF-8 `OsStr`, `stock_term_strings()`);
* added compile-time error when incompatible `Truthy` implementation features are combined;
* README and **Cargo.toml** documentation improvements;


## 0.0.6 - 1st September 2025

* GitHub Actions;
* documentation;
* badges;
* .gitattributes;


## 0.0.5 - 10th August 2025

* added conditional-support for `CStr`, `CString`, `OsStr`, `OsString`;


## 0.0.4 - 10th August 2025

* expanded and improved implementations of `Truthy` trait;


## 0.0.3 - 10th August 2025

* clippy;


## 0.0.2 - 10th August 2025

* added `#is_truthy()` to `Truthy` trait;
* added `get_stock_terms()`;
* project boilerplate;


## 0.0.1 - 10th August 2025

* first release of API (`string_is_falsey()`, `string_is_truey()`, `string_is_truthy()`, `string_is_truthy_with()`, `Terms`, `Truthy`);



<!-- ########################### end of file ########################### -->

