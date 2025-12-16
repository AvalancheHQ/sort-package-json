use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

const SMALL_PACKAGE_JSON: &str = r#"{
  "version": "1.0.0",
  "name": "small-package",
  "dependencies": {
    "lodash": "^4.17.21"
  }
}"#;

const MEDIUM_PACKAGE_JSON: &str = r#"{
  "name": "medium-package",
  "version": "2.5.0",
  "description": "A medium-sized package for benchmarking",
  "keywords": ["test", "benchmark", "sorting"],
  "homepage": "https://github.com/test/package",
  "bugs": {
    "url": "https://github.com/test/package/issues"
  },
  "license": "MIT",
  "author": {
    "name": "Test Author",
    "email": "test@example.com"
  },
  "main": "./dist/index.js",
  "types": "./dist/index.d.ts",
  "files": ["dist", "src"],
  "scripts": {
    "build": "tsc",
    "test": "jest",
    "lint": "eslint ."
  },
  "dependencies": {
    "react": "^18.0.0",
    "axios": "^1.0.0"
  },
  "devDependencies": {
    "typescript": "^5.0.0",
    "jest": "^29.0.0"
  },
  "engines": {
    "node": ">=18.0.0"
  }
}"#;

const LARGE_PACKAGE_JSON: &str = include_str!("../tests/fixtures/package.json");

fn bench_sort_small(c: &mut Criterion) {
    c.bench_function("sort_small_package_json", |b| {
        b.iter(|| sort_package_json(SMALL_PACKAGE_JSON))
    });
}

fn bench_sort_medium(c: &mut Criterion) {
    c.bench_function("sort_medium_package_json", |b| {
        b.iter(|| sort_package_json(MEDIUM_PACKAGE_JSON))
    });
}

fn bench_sort_large(c: &mut Criterion) {
    c.bench_function("sort_large_package_json", |b| {
        b.iter(|| sort_package_json(LARGE_PACKAGE_JSON))
    });
}

fn bench_sort_already_sorted(c: &mut Criterion) {
    // First sort it to get the sorted version
    let sorted = sort_package_json(LARGE_PACKAGE_JSON).unwrap();
    
    c.bench_function("sort_already_sorted_package_json", |b| {
        b.iter(|| sort_package_json(&sorted))
    });
}

criterion_group!(
    benches,
    bench_sort_small,
    bench_sort_medium,
    bench_sort_large,
    bench_sort_already_sorted
);
criterion_main!(benches);
