// benches/string_truthy.rs : benchmarking string truthy evaluation
//
// Requires default Criterion features (including `plotters`; see Cargo.toml).
// After `cargo bench`, open the grouped HTML report:
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
    classify : C,
) where
    M : criterion::measurement::Measurement,
    C : Fn(&str) + Copy,
{
    for (input, label) in INPUTS {
        group.bench_with_input(benchmark_id(input, label), input, |b, input| {
            b.iter(|| {
                classify(input);
            })
        });
    }
}

fn bench_string_is_truthy(c : &mut Criterion) {
    let mut group = c.benchmark_group("string_is_truthy");

    bench_inputs(&mut group, |s| {
        let _ = string_is_truthy(s);
    });

    group.finish();
}

fn bench_string_is_truey(c : &mut Criterion) {
    let mut group = c.benchmark_group("string_is_truey");

    bench_inputs(&mut group, |s| {
        let _ = string_is_truey(s);
    });

    group.finish();
}

fn bench_string_is_falsey(c : &mut Criterion) {
    let mut group = c.benchmark_group("string_is_falsey");

    bench_inputs(&mut group, |s| {
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

criterion_group!(
    benches,
    bench_string_is_truthy,
    bench_string_is_truey,
    bench_string_is_falsey,
    bench_mixed_inputs,
);
criterion_main!(benches);


// ///////////////////////////// end of file //////////////////////////// //
