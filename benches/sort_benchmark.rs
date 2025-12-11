use criterion::{Criterion, black_box, criterion_group, criterion_main};
use sort_package_json::sort_package_json;
use std::fs;

fn bench_sort_small_package_json(c: &mut Criterion) {
    let input =
        fs::read_to_string("tests/fixtures/package.json").expect("Failed to read fixture file");

    c.bench_function("sort_small_package_json", |b| {
        b.iter(|| sort_package_json(black_box(&input)).expect("Failed to sort package.json"))
    });
}

fn bench_sort_idempotent(c: &mut Criterion) {
    let input =
        fs::read_to_string("tests/fixtures/package.json").expect("Failed to read fixture file");
    let sorted_once = sort_package_json(&input).expect("Failed to sort package.json");

    c.bench_function("sort_idempotent", |b| {
        b.iter(|| sort_package_json(black_box(&sorted_once)).expect("Failed to sort package.json"))
    });
}

fn bench_parse_only(c: &mut Criterion) {
    let input =
        fs::read_to_string("tests/fixtures/package.json").expect("Failed to read fixture file");

    c.bench_function("parse_json_only", |b| {
        b.iter(|| {
            let _: serde_json::Value =
                serde_json::from_str(black_box(&input)).expect("Failed to parse JSON");
        })
    });
}

criterion_group!(benches, bench_sort_small_package_json, bench_sort_idempotent, bench_parse_only);
criterion_main!(benches);
