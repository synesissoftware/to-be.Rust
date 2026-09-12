# to-be.Rust - Changes <!-- omit in toc -->


## 0.0.10 - 12th September 2026

* removed the Rust-specific Cursor rule superseded by shared workspace standards;
* strengthened CI with stable checks, pinned nightly formatting, example builds, and package validation;
* refreshed repository metadata, editor configuration, ignore rules, and Rust formatting configuration;
* added the **versions** example and excluded development-only files from published packages;
* documented the MSRV and canonicalised **Cargo.toml** feature and dependency metadata;
* improved the test-name checker and refreshed the formatting driver;


## 0.0.9 - 31st August 2026

* aligned **.github/workflows/ci.yml** with the canonical branch, permission, locked feature, toolchain, and release checks;
* added **DOC_76**, **RUST_TEST_NAMING**, and **DERIVE_LAYOUT** repository checkers, together with AsStr feature coverage;
* added **EXAMPLES.md** and **NEWS.md**, and aligned README terminology, crate documentation, docs.rs configuration, and package contents;
* updated the **base-traits** 0.1.x dependency, refreshed **Cargo.lock**, and pinned nightly formatter policy;


## 0.0.8 - 29th June 2026

* optimised string truthiness parsing with conditional trim (leading/trailing ASCII whitespace lookahead before `trim()` on the precise-match fast path);
* expanded unit tests for whitespace-padded stock terms (including single-digit `" 1"`, `"1 "`, and tab padding);
* extended **benches/string_truthy** Criterion benchmarks (custom `Terms`, padded stock inputs);


## 0.0.7 - 27th June 2026

* upgraded CI (Clippy, rustfmt, MSRV job, optional-feature test matrix);
* added crate-level rustdoc;
* aligned **rustfmt.toml** with **Diagnosticism.Rust** / SIS conventions;
* added **benches/string_truthy** Criterion benchmark;
* added **examples/truthy_strings** example program;
* modularised implementation (`constants`, `parse`, `terms`, `truthy`, `impls`);
* added `os_string_is_truthy()` (fixes missing symbol for `OsStr` / `OsString` features);
* expanded tests (stock tables, `CStr`, non-UTF-8 `OsStr`, `stock_term_strings()`);
* added compile-time error when incompatible `Truthy` implementation features are combined;
* improved **README.md** and **Cargo.toml** documentation;


## 0.0.6 - 1st September 2025

* added GitHub Actions;
* added documentation;
* added badges;
* added **.gitattributes**;


## 0.0.5 - 10th August 2025

* added conditional support for `CStr`, `CString`, `OsStr`, and `OsString`;


## 0.0.4 - 10th August 2025

* expanded and improved implementations of the `Truthy` trait;


## 0.0.3 - 10th August 2025

* added Clippy checks;


## 0.0.2 - 10th August 2025

* added `#is_truthy()` to the `Truthy` trait;
* added `get_stock_terms()`;
* added project boilerplate;


## 0.0.1 - 10th August 2025

* released the initial API (`string_is_falsey()`, `string_is_truey()`, `string_is_truthy()`, `string_is_truthy_with()`, `Terms`, and `Truthy`);



<!-- ########################### end of file ########################### -->

