use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

const SMALL_PACKAGE_JSON: &str = r#"{
  "name": "small-package",
  "version": "1.0.0",
  "scripts": {
    "test": "jest",
    "build": "webpack"
  },
  "dependencies": {
    "react": "^18.0.0"
  }
}"#;

const LARGE_PACKAGE_JSON: &str = include_str!("../tests/fixtures/package.json");

fn bench_sort_small_package(c: &mut Criterion) {
    c.bench_function("sort_small_package", |b| {
        b.iter(|| {
            sort_package_json(black_box(SMALL_PACKAGE_JSON)).unwrap();
        });
    });
}

fn bench_sort_large_package(c: &mut Criterion) {
    c.bench_function("sort_large_package", |b| {
        b.iter(|| {
            sort_package_json(black_box(LARGE_PACKAGE_JSON)).unwrap();
        });
    });
}

fn bench_sort_idempotency(c: &mut Criterion) {
    let sorted = sort_package_json(LARGE_PACKAGE_JSON).unwrap();
    c.bench_function("sort_idempotency", |b| {
        b.iter(|| {
            sort_package_json(black_box(&sorted)).unwrap();
        });
    });
}

fn bench_parse_only(c: &mut Criterion) {
    c.bench_function("parse_only", |b| {
        b.iter(|| {
            let _: serde_json::Value = serde_json::from_str(black_box(LARGE_PACKAGE_JSON)).unwrap();
        });
    });
}

criterion_group!(
    benches,
    bench_sort_small_package,
    bench_sort_large_package,
    bench_sort_idempotency,
    bench_parse_only
);
criterion_main!(benches);
