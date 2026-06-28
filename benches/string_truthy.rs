// benches/string_truthy.rs : benchmarking string truthy evaluation

#![allow(non_snake_case)]

use criterion::{
    BatchSize,
    Criterion,
    criterion_group,
    criterion_main,
};
use to_be::{
    string_is_falsey,
    string_is_truey,
    string_is_truthy,
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

fn bench_string_is_truthy(c : &mut Criterion) {

    for s in INPUTS {
        let (s, t) = s;

        if t.is_empty() {
            c.bench_function(format!("string_is_truthy/{}", s).as_str(), |b| {
                b.iter(|| string_is_truthy(s))
            });
        } else {
            c.bench_function(format!("string_is_truthy/{} - {}", s, t).as_str(), |b| {
                b.iter(|| string_is_truthy(s))
            });
        }
    }
}

fn bench_string_is_truey(c : &mut Criterion) {

    for s in INPUTS {
        let (s, t) = s;

        if t.is_empty() {
            c.bench_function(format!("string_is_truey/{}", s).as_str(), |b| {
                b.iter(|| string_is_truey(s))
            });
        } else {
            c.bench_function(format!("string_is_truey/{} - {}", s, t).as_str(), |b| {
                b.iter(|| string_is_truey(s))
            });
        }
    }
}

fn bench_string_is_falsey(c : &mut Criterion) {
    for s in INPUTS {
        let (s, t) = s;

        if t.is_empty() {
            c.bench_function(format!("string_is_falsey/{}", s).as_str(), |b| {
                b.iter(|| string_is_falsey(s))
            });
        } else {
            c.bench_function(format!("string_is_falsey/{} - {}", s, t).as_str(), |b| {
                b.iter(|| string_is_falsey(s))
            });
        }
    }
}

fn bench_mixed_inputs(c : &mut Criterion) {
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

    c.bench_function("string_is_truthy/mixed_batch", |b| {
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
}

criterion_group!(
    benches,
    bench_string_is_truthy,
    bench_string_is_truey,
    bench_string_is_falsey,
    bench_mixed_inputs,
);
criterion_main!(benches);


// ///////////////////////////// end of file //////////////////////////// //
