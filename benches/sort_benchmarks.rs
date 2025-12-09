use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;
use std::fs;

fn bench_sort_simple_package_json(c: &mut Criterion) {
    let simple_json = r#"{
  "version": "1.0.0",
  "name": "simple-package",
  "description": "A simple package",
  "dependencies": {
    "react": "^18.0.0"
  }
}"#;

    c.bench_function("sort_simple_package_json", |b| {
        b.iter(|| sort_package_json(simple_json))
    });
}

fn bench_sort_complex_package_json(c: &mut Criterion) {
    // Load the complex fixture from the tests directory
    let complex_json = fs::read_to_string("tests/fixtures/package.json")
        .expect("Failed to read fixture file");

    c.bench_function("sort_complex_package_json", |b| {
        b.iter(|| sort_package_json(&complex_json))
    });
}

fn bench_sort_large_dependencies(c: &mut Criterion) {
    let large_deps_json = r#"{
  "name": "large-deps-package",
  "version": "1.0.0",
  "dependencies": {
    "zod": "^3.0.0",
    "react": "^18.0.0",
    "axios": "^1.0.0",
    "lodash": "^4.17.21",
    "typescript": "^5.0.0",
    "webpack": "^5.0.0",
    "express": "^4.18.0",
    "mongoose": "^7.0.0",
    "dotenv": "^16.0.0",
    "jest": "^29.0.0",
    "eslint": "^8.0.0",
    "prettier": "^3.0.0",
    "tailwindcss": "^3.0.0",
    "vite": "^4.0.0",
    "vitest": "^0.34.0"
  },
  "devDependencies": {
    "husky": "^8.0.0",
    "commitlint": "^17.0.0",
    "@types/node": "^20.0.0",
    "@types/react": "^18.0.0"
  }
}"#;

    c.bench_function("sort_large_dependencies", |b| {
        b.iter(|| sort_package_json(large_deps_json))
    });
}

fn bench_sort_with_scripts(c: &mut Criterion) {
    let scripts_json = r#"{
  "name": "scripts-package",
  "version": "1.0.0",
  "scripts": {
    "test": "jest",
    "build": "webpack",
    "dev": "webpack serve",
    "lint": "eslint .",
    "format": "prettier --write .",
    "pretest": "echo 'Starting tests'",
    "posttest": "echo 'Tests complete'",
    "prebuild": "npm run lint",
    "postbuild": "echo 'Build complete'",
    "start": "node index.js",
    "clean": "rm -rf dist"
  }
}"#;

    c.bench_function("sort_with_scripts", |b| {
        b.iter(|| sort_package_json(scripts_json))
    });
}

fn bench_sort_with_exports(c: &mut Criterion) {
    let exports_json = r#"{
  "name": "exports-package",
  "version": "1.0.0",
  "exports": {
    ".": {
      "import": "./dist/index.esm.js",
      "require": "./dist/index.cjs",
      "types": "./dist/index.d.ts",
      "default": "./dist/index.js"
    },
    "./utils": {
      "import": "./dist/utils.esm.js",
      "types": "./dist/utils.d.ts",
      "default": "./dist/utils.js"
    },
    "./package.json": "./package.json"
  }
}"#;

    c.bench_function("sort_with_exports", |b| {
        b.iter(|| sort_package_json(exports_json))
    });
}

fn bench_sort_idempotent(c: &mut Criterion) {
    let json = r#"{
  "version": "1.0.0",
  "name": "test-package",
  "dependencies": {
    "react": "^18.0.0"
  }
}"#;

    // Pre-sort once
    let sorted = sort_package_json(json).expect("Failed to sort");

    c.bench_function("sort_idempotent", |b| {
        b.iter(|| sort_package_json(&sorted))
    });
}

criterion_group!(
    benches,
    bench_sort_simple_package_json,
    bench_sort_complex_package_json,
    bench_sort_large_dependencies,
    bench_sort_with_scripts,
    bench_sort_with_exports,
    bench_sort_idempotent
);
criterion_main!(benches);
