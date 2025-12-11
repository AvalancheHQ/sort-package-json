use criterion::{Criterion, black_box, criterion_group, criterion_main};
use sort_package_json::sort_package_json;
use std::fs;

fn bench_sort_simple_package(c: &mut Criterion) {
    let input = r#"{
  "version": "1.0.0",
  "dependencies": {
    "react": "^18.0.0"
  },
  "name": "simple-package",
  "scripts": {
    "test": "jest",
    "build": "webpack"
  }
}"#;

    c.bench_function("sort_simple_package", |b| b.iter(|| sort_package_json(black_box(input))));
}

fn bench_sort_complex_package(c: &mut Criterion) {
    let input = fs::read_to_string("tests/fixtures/package.json").expect("Failed to read fixture");

    c.bench_function("sort_complex_package", |b| b.iter(|| sort_package_json(black_box(&input))));
}

fn bench_sort_minimal_package(c: &mut Criterion) {
    let input = r#"{
  "name": "minimal",
  "version": "1.0.0"
}"#;

    c.bench_function("sort_minimal_package", |b| b.iter(|| sort_package_json(black_box(input))));
}

fn bench_sort_large_dependencies(c: &mut Criterion) {
    let input = r#"{
  "name": "large-deps",
  "version": "1.0.0",
  "dependencies": {
    "axios": "^1.0.0",
    "express": "^4.18.0",
    "lodash": "^4.17.21",
    "moment": "^2.29.0",
    "react": "^18.0.0",
    "react-dom": "^18.0.0",
    "redux": "^4.2.0",
    "typescript": "^5.0.0",
    "webpack": "^5.0.0",
    "zod": "^3.0.0"
  },
  "devDependencies": {
    "jest": "^29.0.0",
    "eslint": "^8.0.0",
    "prettier": "^2.8.0",
    "typescript": "^5.0.0"
  },
  "scripts": {
    "test": "jest",
    "build": "tsc",
    "lint": "eslint .",
    "format": "prettier --write ."
  }
}"#;

    c.bench_function("sort_large_dependencies", |b| b.iter(|| sort_package_json(black_box(input))));
}

fn bench_sort_with_nested_config(c: &mut Criterion) {
    let input = r#"{
  "name": "nested-config",
  "version": "1.0.0",
  "jest": {
    "testEnvironment": "node",
    "coverageDirectory": "coverage",
    "collectCoverageFrom": ["src/**/*.{js,jsx}"],
    "testMatch": ["**/__tests__/**/*.js"]
  },
  "babel": {
    "presets": ["@babel/preset-env", "@babel/preset-react"],
    "plugins": ["@babel/plugin-proposal-class-properties"]
  },
  "eslintConfig": {
    "extends": ["eslint:recommended"],
    "rules": {
      "no-console": "warn"
    }
  }
}"#;

    c.bench_function("sort_with_nested_config", |b| b.iter(|| sort_package_json(black_box(input))));
}

fn bench_sort_idempotency(c: &mut Criterion) {
    let input = fs::read_to_string("tests/fixtures/package.json").expect("Failed to read fixture");
    let sorted = sort_package_json(&input).expect("Failed to sort");

    c.bench_function("sort_idempotency", |b| b.iter(|| sort_package_json(black_box(&sorted))));
}

criterion_group!(
    benches,
    bench_sort_simple_package,
    bench_sort_complex_package,
    bench_sort_minimal_package,
    bench_sort_large_dependencies,
    bench_sort_with_nested_config,
    bench_sort_idempotency
);
criterion_main!(benches);
