use criterion::{Criterion, black_box, criterion_group, criterion_main};
use sort_package_json::sort_package_json;
use std::fs;

fn bench_sort_package_json(c: &mut Criterion) {
    let input =
        fs::read_to_string("tests/fixtures/package.json").expect("Failed to read fixture file");

    c.bench_function("sort_package_json", |b| b.iter(|| sort_package_json(black_box(&input))));
}

fn bench_sort_idempotency(c: &mut Criterion) {
    let input =
        fs::read_to_string("tests/fixtures/package.json").expect("Failed to read fixture file");
    let sorted = sort_package_json(&input).expect("Failed to sort on first pass");

    c.bench_function("sort_package_json_idempotent", |b| {
        b.iter(|| sort_package_json(black_box(&sorted)))
    });
}

fn bench_sort_large_package(c: &mut Criterion) {
    // Create a larger package.json for benchmarking
    let large_json = r#"{
  "name": "large-test-package",
  "version": "1.0.0",
  "description": "A large test package with many dependencies",
  "keywords": ["test", "sorting", "json", "benchmark", "performance", "large", "dependencies"],
  "license": "MIT",
  "scripts": {
    "test": "jest",
    "build": "webpack",
    "lint": "eslint .",
    "dev": "webpack serve",
    "pretest": "echo 'Starting tests'",
    "posttest": "echo 'Tests complete'",
    "prebuild": "rimraf dist",
    "postbuild": "echo 'Build complete'",
    "predeploy": "npm run build",
    "deploy": "gh-pages -d dist",
    "start": "node index.js",
    "format": "prettier --write .",
    "typecheck": "tsc --noEmit"
  },
  "dependencies": {
    "react": "^18.0.0",
    "axios": "^1.0.0",
    "lodash": "^4.17.21",
    "express": "^4.18.0",
    "redux": "^4.2.0",
    "react-redux": "^8.0.0",
    "react-router-dom": "^6.0.0",
    "typescript": "^5.0.0",
    "webpack": "^5.0.0",
    "babel-loader": "^9.0.0",
    "css-loader": "^6.0.0",
    "style-loader": "^3.0.0",
    "ts-loader": "^9.0.0",
    "html-webpack-plugin": "^5.0.0"
  },
  "devDependencies": {
    "jest": "^29.0.0",
    "eslint": "^8.0.0",
    "prettier": "^3.0.0",
    "@types/react": "^18.0.0",
    "@types/node": "^20.0.0",
    "@typescript-eslint/eslint-plugin": "^6.0.0",
    "@typescript-eslint/parser": "^6.0.0",
    "rimraf": "^5.0.0",
    "gh-pages": "^6.0.0"
  }
}"#;

    c.bench_function("sort_large_package_json", |b| {
        b.iter(|| sort_package_json(black_box(large_json)))
    });
}

criterion_group!(
    benches,
    bench_sort_package_json,
    bench_sort_idempotency,
    bench_sort_large_package
);
criterion_main!(benches);
