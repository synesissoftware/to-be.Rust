// benches/string_truthy.rs : benchmarking string truthy evaluation
//
// Requires default Criterion features (including `plotters`; see Cargo.toml).
//
// Run only this harness (Criterion flags are not accepted by `cargo bench`
// on the library unit-test target):
//
//   cargo bench --bench string_truthy
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

criterion_group!(
    benches,
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
