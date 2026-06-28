// benches/string_truthy.rs : benchmarking string truthy evaluation
//
// Requires default Criterion features (including `plotters`; see Cargo.toml).
//
// Run only this harness (Criterion flags are not accepted by `cargo bench`
// on the library unit-test target):
//
//   cargo bench --bench string_truthy
//
// Run only the custom-terms benchmarks (much faster than the full suite):
//
//   cargo bench --bench string_truthy -- custom_terms
//
// Normative custom-terms only (excludes leading/trailing/both padded cases):
//
//   cargo bench --bench string_truthy -- 'custom_terms/(falsey|truey|stock_miss|unrecognised|empty|mixed_batch$)'
//
// Padded custom-terms only:
//
//   cargo bench --bench string_truthy -- 'custom_terms/(leading|trailing|both|mixed_batch_padded)'
//
// Save or compare a baseline:
//
//   cargo bench --bench string_truthy -- --save-baseline baseline
//   cargo bench --bench string_truthy -- --baseline baseline
//
// After `cargo bench --bench string_truthy`, open the grouped HTML report:
//   open target/criterion/report/index.html

#![allow(non_snake_case)]

use to_be::{
    string_is_falsey,
    string_is_truey,
    string_is_truthy,
    string_is_truthy_with,
    Terms,
};

use criterion::{
    criterion_group,
    criterion_main,
    BatchSize,
    BenchmarkId,
    Criterion,
};


const INPUTS : [(&str, &str); 21] = [
    // true / false
    ("true", "lower"),
    ("TRUE", "upper"),
    ("True", "title"),
    ("false", "lower"),
    ("FALSE", "upper"),
    ("False", "title"),
    // yes / no
    ("yes", "lower"),
    ("YES", "upper"),
    ("Yes", "title"),
    ("no", "lower"),
    ("NO", "upper"),
    ("No", "title"),
    // on / off
    ("on", "lower"),
    ("ON", "upper"),
    ("On", "title"),
    ("off", "lower"),
    ("OFF", "upper"),
    ("Off", "title"),
    // 1 / 0
    ("1", ""),
    ("0", ""),
    // other
    ("unrecognised", "unrecognised"),
];

macro_rules! padded_inputs {
    ($(
        $term : literal / $id : literal
    ),* $(,)?) => {
        [
            $(
                (concat!(" ", $term), concat!("leading/", $id)),
                (concat!($term, " "), concat!("trailing/", $id)),
                (concat!(" ", $term, " "), concat!("both/", $id)),
            )*
        ]
    };
}

#[rustfmt::skip]
const PADDED_INPUTS : [(&str, &str); 63] = padded_inputs![
    "true" / "lower/true",
    "TRUE" / "upper/TRUE",
    "True" / "title/True",
    "false" / "lower/false",
    "FALSE" / "upper/FALSE",
    "False" / "title/False",
    "yes" / "lower/yes",
    "YES" / "upper/YES",
    "Yes" / "title/Yes",
    "no" / "lower/no",
    "NO" / "upper/NO",
    "No" / "title/No",
    "on" / "lower/on",
    "ON" / "upper/ON",
    "On" / "title/On",
    "off" / "lower/off",
    "OFF" / "upper/OFF",
    "Off" / "title/Off",
    "1" / "1",
    "0" / "0",
    "unrecognised" / "unrecognised",
];

#[rustfmt::skip]
const CUSTOM_TRUEY_PRECISE_STRINGS : &[&str] = &[
    "Da",
    "YUP",
    "Yup",
];

#[rustfmt::skip]
const CUSTOM_TRUEY_LOWERCASE_STRINGS : &[&str] = &[
    "da",
    "yup",
];

#[rustfmt::skip]
const CUSTOM_FALSEY_PRECISE_STRINGS : &[&str] = &[
    "Nyet",
    "Nope",
];

#[rustfmt::skip]
const CUSTOM_FALSEY_LOWERCASE_STRINGS : &[&str] = &[
    "nyet",
    "nope",
];

const CUSTOM_TERMS : Terms<'static> = Terms::Strings {
    falsey_precise_strings :   CUSTOM_FALSEY_PRECISE_STRINGS,
    falsey_lowercase_strings : CUSTOM_FALSEY_LOWERCASE_STRINGS,
    truey_precise_strings :    CUSTOM_TRUEY_PRECISE_STRINGS,
    truey_lowercase_strings :  CUSTOM_TRUEY_LOWERCASE_STRINGS,
};

#[rustfmt::skip]
const CUSTOM_INPUTS : [(&str, &str); 17] = [
    // custom falsey
    ("Nyet", "falsey/precise/Nyet"),
    ("NYET", "falsey/upper/NYET"),
    ("nyET", "falsey/mixed/nyET"),
    ("nope", "falsey/lower/nope"),
    ("Nope", "falsey/title/Nope"),
    ("NOPE", "falsey/upper/NOPE"),
    // custom truey
    ("Da", "truey/precise/Da"),
    ("YUP", "truey/precise/YUP"),
    ("Yup", "truey/title/Yup"),
    ("DA", "truey/upper/DA"),
    ("dA", "truey/mixed/dA"),
    ("yup", "truey/lower/yup"),
    ("yUp", "truey/mixed/yUp"),
    // stock strings (not in custom tables)
    ("true", "stock_miss/true"),
    ("false", "stock_miss/false"),
    // other
    ("maybe", "unrecognised/maybe"),
    ("", "empty"),
];

#[rustfmt::skip]
const CUSTOM_PADDED_INPUTS : [(&str, &str); 45] = padded_inputs![
    "Nyet" / "falsey/precise/Nyet",
    "NYET" / "falsey/upper/NYET",
    "nyET" / "falsey/mixed/nyET",
    "nope" / "falsey/lower/nope",
    "Nope" / "falsey/title/Nope",
    "NOPE" / "falsey/upper/NOPE",
    "Da" / "truey/precise/Da",
    "YUP" / "truey/precise/YUP",
    "Yup" / "truey/title/Yup",
    "DA" / "truey/upper/DA",
    "dA" / "truey/mixed/dA",
    "yup" / "truey/lower/yup",
    "yUp" / "truey/mixed/yUp",
    "true" / "stock_miss/true",
    "maybe" / "unrecognised/maybe",
];


fn benchmark_id(
    input : &str,
    label : &str,
) -> BenchmarkId {
    if label.is_empty() {
        BenchmarkId::from_parameter(input)
    } else {
        BenchmarkId::new(label, input)
    }
}

fn bench_inputs<M, C>(
    group : &mut criterion::BenchmarkGroup<'_, M>,
    inputs : &[(&str, &str)],
    classify : C,
) where
    M : criterion::measurement::Measurement,
    C : Fn(&str) + Copy,
{
    for &(input, label) in inputs {
        group.bench_with_input(benchmark_id(input, label), input, |b, input| {
            b.iter(|| {
                classify(input);
            })
        });
    }
}

fn bench_custom_terms_inputs<M>(
    group : &mut criterion::BenchmarkGroup<'_, M>,
    inputs : &[(&str, &str)],
) where
    M : criterion::measurement::Measurement,
{
    for &(input, label) in inputs {
        group.bench_with_input(benchmark_id(input, label), input, |b, input| {
            b.iter(|| {
                let _ = string_is_truthy_with(input, CUSTOM_TERMS);
            })
        });
    }
}

fn bench_string_is_truthy(c : &mut Criterion) {
    let mut group = c.benchmark_group("string_is_truthy");

    bench_inputs(&mut group, &INPUTS, |s| {
        let _ = string_is_truthy(s);
    });

    group.finish();
}

fn bench_string_is_truey(c : &mut Criterion) {
    let mut group = c.benchmark_group("string_is_truey");

    bench_inputs(&mut group, &INPUTS, |s| {
        let _ = string_is_truey(s);
    });

    group.finish();
}

fn bench_string_is_falsey(c : &mut Criterion) {
    let mut group = c.benchmark_group("string_is_falsey");

    bench_inputs(&mut group, &INPUTS, |s| {
        let _ = string_is_falsey(s);
    });

    group.finish();
}

fn bench_string_is_truthy_padded(c : &mut Criterion) {
    let mut group = c.benchmark_group("string_is_truthy_padded");

    bench_inputs(&mut group, &PADDED_INPUTS, |s| {
        let _ = string_is_truthy(s);
    });

    group.finish();
}

fn bench_string_is_truey_padded(c : &mut Criterion) {
    let mut group = c.benchmark_group("string_is_truey_padded");

    bench_inputs(&mut group, &PADDED_INPUTS, |s| {
        let _ = string_is_truey(s);
    });

    group.finish();
}

fn bench_string_is_falsey_padded(c : &mut Criterion) {
    let mut group = c.benchmark_group("string_is_falsey_padded");

    bench_inputs(&mut group, &PADDED_INPUTS, |s| {
        let _ = string_is_falsey(s);
    });

    group.finish();
}

fn bench_mixed_inputs(c : &mut Criterion) {
    #[rustfmt::skip]
    let inputs = [
        "yes",
        "no",
        "TRUE",
        "off",
        "maybe",
        "1",
        "0",
        "",
    ];

    let mut group = c.benchmark_group("string_is_truthy");

    group.bench_function("mixed_batch", |b| {
        b.iter_batched(
            || inputs.as_slice(),
            |slice| {
                for s in slice {
                    let _ = string_is_truthy(s);
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_mixed_inputs_padded(c : &mut Criterion) {
    #[rustfmt::skip]
    let inputs = [
        " yes ",
        " no ",
        " TRUE ",
        " off ",
        " maybe ",
        " 1 ",
        " 0 ",
        "   ",
    ];

    let mut group = c.benchmark_group("string_is_truthy_padded");

    group.bench_function("mixed_batch", |b| {
        b.iter_batched(
            || inputs.as_slice(),
            |slice| {
                for s in slice {
                    let _ = string_is_truthy(s);
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_custom_terms(c : &mut Criterion) {
    let mut group = c.benchmark_group("custom_terms");

    bench_custom_terms_inputs(&mut group, &CUSTOM_INPUTS);

    #[rustfmt::skip]
    let mixed = [
        "Nyet",
        "Da",
        "yup",
        "nope",
        "maybe",
        "true",
        "",
    ];

    group.bench_function("mixed_batch", |b| {
        b.iter_batched(
            || mixed.as_slice(),
            |slice| {
                for s in slice {
                    let _ = string_is_truthy_with(s, CUSTOM_TERMS);
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_custom_terms_padded(c : &mut Criterion) {
    let mut group = c.benchmark_group("custom_terms");

    bench_custom_terms_inputs(&mut group, &CUSTOM_PADDED_INPUTS);

    #[rustfmt::skip]
    let mixed_padded = [
        " Nyet ",
        " Da ",
        " yup ",
        " nope ",
        " maybe ",
        " true ",
        "   ",
    ];

    group.bench_function("mixed_batch_padded", |b| {
        b.iter_batched(
            || mixed_padded.as_slice(),
            |slice| {
                for s in slice {
                    let _ = string_is_truthy_with(s, CUSTOM_TERMS);
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_custom_terms,
    bench_custom_terms_padded,
    bench_mixed_inputs,
    bench_mixed_inputs_padded,
    bench_string_is_truthy,
    bench_string_is_truthy_padded,
    bench_string_is_truey,
    bench_string_is_truey_padded,
    bench_string_is_falsey,
    bench_string_is_falsey_padded,
);
criterion_main!(benches);


// ///////////////////////////// end of file //////////////////////////// //
