use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;
use std::fs;

/// Benchmark for sorting a small package.json file
fn bench_small_package_json(c: &mut Criterion) {
    let input =
        fs::read_to_string("tests/fixtures/package.json").expect("Failed to read fixture file");

    c.bench_function("sort_small_package_json", |b| {
        b.iter(|| sort_package_json(black_box(&input)).unwrap())
    });
}

/// Benchmark for sorting a minimal package.json file
fn bench_minimal_package_json(c: &mut Criterion) {
    let minimal_json = r#"{
  "version": "1.0.0",
  "name": "minimal-package"
}"#;

    c.bench_function("sort_minimal_package_json", |b| {
        b.iter(|| sort_package_json(black_box(minimal_json)).unwrap())
    });
}

/// Benchmark for sorting a large package.json file with many dependencies
fn bench_large_package_json(c: &mut Criterion) {
    let large_json = r#"{
  "name": "large-package",
  "version": "2.0.0",
  "description": "A large package with many fields",
  "keywords": ["test", "large", "benchmark", "performance", "json", "sort"],
  "homepage": "https://github.com/test/large-package",
  "bugs": {
    "url": "https://github.com/test/large-package/issues",
    "email": "bugs@example.com"
  },
  "license": "MIT",
  "author": {
    "name": "Author Name",
    "email": "author@example.com",
    "url": "https://example.com"
  },
  "main": "./dist/index.js",
  "module": "./dist/index.esm.js",
  "types": "./dist/index.d.ts",
  "exports": {
    ".": {
      "import": "./dist/index.esm.js",
      "require": "./dist/index.cjs",
      "types": "./dist/index.d.ts"
    }
  },
  "scripts": {
    "build": "tsc",
    "test": "jest",
    "lint": "eslint .",
    "format": "prettier --write .",
    "pretest": "npm run lint",
    "posttest": "npm run coverage",
    "dev": "webpack serve",
    "start": "node dist/index.js"
  },
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "axios": "^1.4.0",
    "lodash": "^4.17.21",
    "express": "^4.18.2",
    "mongoose": "^7.0.0",
    "dotenv": "^16.0.3",
    "cors": "^2.8.5",
    "body-parser": "^1.20.2",
    "jsonwebtoken": "^9.0.0"
  },
  "devDependencies": {
    "typescript": "^5.0.0",
    "jest": "^29.5.0",
    "eslint": "^8.40.0",
    "prettier": "^2.8.8",
    "webpack": "^5.82.0",
    "webpack-cli": "^5.1.0",
    "@types/node": "^20.0.0",
    "@types/react": "^18.2.0",
    "@types/express": "^4.17.17",
    "ts-node": "^10.9.1"
  },
  "peerDependencies": {
    "react": ">=18.0.0"
  },
  "engines": {
    "node": ">=18.0.0",
    "npm": ">=9.0.0"
  }
}"#;

    c.bench_function("sort_large_package_json", |b| {
        b.iter(|| sort_package_json(black_box(large_json)).unwrap())
    });
}

/// Benchmark for sorting with many custom fields
fn bench_custom_fields(c: &mut Criterion) {
    let custom_json = r#"{
  "name": "custom-package",
  "version": "1.0.0",
  "customField1": "value1",
  "customField2": "value2",
  "customField3": "value3",
  "customField4": "value4",
  "customField5": "value5",
  "_private1": "private1",
  "_private2": "private2",
  "_private3": "private3",
  "description": "Package with many custom fields",
  "scripts": {
    "test": "jest"
  }
}"#;

    c.bench_function("sort_custom_fields", |b| {
        b.iter(|| sort_package_json(black_box(custom_json)).unwrap())
    });
}

/// Benchmark for sorting already sorted package.json (best case)
fn bench_already_sorted(c: &mut Criterion) {
    let sorted_json = r#"{
  "name": "sorted-package",
  "version": "1.0.0",
  "description": "Already sorted",
  "scripts": {
    "test": "jest"
  },
  "dependencies": {
    "react": "^18.0.0"
  }
}"#;

    c.bench_function("sort_already_sorted", |b| {
        b.iter(|| sort_package_json(black_box(sorted_json)).unwrap())
    });
}

criterion_group!(
    benches,
    bench_small_package_json,
    bench_minimal_package_json,
    bench_large_package_json,
    bench_custom_fields,
    bench_already_sorted
);
criterion_main!(benches);
