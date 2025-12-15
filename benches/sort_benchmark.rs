use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

fn benchmark_sort_package_json(c: &mut Criterion) {
    let simple_input = r#"{
  "name": "test",
  "version": "1.0.0",
  "dependencies": {
    "lodash": "^4.17.21",
    "axios": "^1.0.0"
  }
}"#;

    c.bench_function("sort_simple_package", |b| {
        b.iter(|| {
            sort_package_json(simple_input).unwrap();
        });
    });

    let complex_input = std::fs::read_to_string("tests/fixtures/package.json")
        .expect("Failed to read fixture");

    c.bench_function("sort_complex_package", |b| {
        b.iter(|| {
            sort_package_json(&complex_input).unwrap();
        });
    });

    c.bench_function("sort_idempotent", |b| {
        let sorted_once = sort_package_json(&complex_input).unwrap();
        b.iter(|| {
            sort_package_json(&sorted_once).unwrap();
        });
    });

    let large_deps = r#"{
  "name": "large-test",
  "version": "1.0.0",
  "dependencies": {
    "zlib": "1.0.0",
    "yaml": "1.0.0",
    "webpack": "5.0.0",
    "typescript": "5.0.0",
    "react": "18.0.0",
    "lodash": "4.17.21",
    "axios": "1.0.0",
    "express": "4.18.0",
    "jest": "29.0.0",
    "eslint": "8.0.0",
    "prettier": "3.0.0",
    "babel": "7.0.0"
  },
  "devDependencies": {
    "vitest": "1.0.0",
    "tsup": "8.0.0",
    "rimraf": "5.0.0",
    "npm-run-all": "4.1.5",
    "husky": "9.0.0",
    "commitlint": "18.0.0",
    "lint-staged": "15.0.0"
  }
}"#;

    c.bench_function("sort_large_dependencies", |b| {
        b.iter(|| {
            sort_package_json(large_deps).unwrap();
        });
    });
}

criterion_group!(benches, benchmark_sort_package_json);
criterion_main!(benches);
