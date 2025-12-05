use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

const SIMPLE_PACKAGE: &str = r#"{
  "version": "1.0.0",
  "dependencies": {},
  "name": "simple-package",
  "scripts": {}
}"#;

const COMPLEX_PACKAGE: &str = r#"{
  "name": "complex-package",
  "version": "2.0.0",
  "description": "A complex package for benchmarking",
  "keywords": ["test", "benchmark", "sorting", "json", "performance"],
  "homepage": "https://github.com/test/complex-package#readme",
  "bugs": {
    "email": "bugs@example.com",
    "url": "https://github.com/test/complex-package/issues"
  },
  "repository": {
    "url": "https://github.com/test/complex-package",
    "type": "git"
  },
  "license": "MIT",
  "author": {
    "name": "Test Author",
    "email": "author@example.com",
    "url": "https://example.com"
  },
  "maintainers": [
    {"name": "Maintainer One", "email": "m1@example.com", "url": "https://m1.com"},
    {"name": "Maintainer Two", "email": "m2@example.com", "url": "https://m2.com"}
  ],
  "exports": {
    ".": {
      "types": "./dist/index.d.ts",
      "import": "./dist/index.esm.js",
      "require": "./dist/index.cjs",
      "default": "./dist/index.js"
    },
    "./package.json": "./package.json",
    "./utils": {
      "types": "./dist/utils.d.ts",
      "import": "./dist/utils.esm.js",
      "default": "./dist/utils.js"
    }
  },
  "main": "./dist/index.js",
  "module": "./dist/index.esm.js",
  "types": "./dist/index.d.ts",
  "files": ["src", "dist", "README.md", "LICENSE"],
  "scripts": {
    "test": "jest",
    "pretest": "echo 'Starting tests'",
    "posttest": "echo 'Tests complete'",
    "build": "webpack",
    "lint": "eslint .",
    "dev": "webpack serve"
  },
  "dependencies": {
    "react": "^18.0.0",
    "axios": "^1.0.0",
    "lodash": "^4.17.21",
    "express": "^4.18.0"
  },
  "devDependencies": {
    "webpack": "^5.0.0",
    "jest": "^29.0.0",
    "typescript": "^5.0.0",
    "eslint": "^8.0.0"
  },
  "peerDependencies": {
    "react": ">=18.0.0"
  },
  "engines": {
    "node": ">=18.0.0",
    "npm": ">=8.0.0"
  },
  "babel": {
    "presets": ["@babel/preset-env", "@babel/preset-react"],
    "plugins": ["@babel/plugin-proposal-class-properties"]
  },
  "publishConfig": {
    "access": "public"
  },
  "customField": "custom value",
  "anotherCustom": "another value",
  "_privateField": "private",
  "_id": "complex-package@2.0.0"
}"#;

fn bench_simple_package(c: &mut Criterion) {
    c.bench_function("sort_simple_package", |b| {
        b.iter(|| sort_package_json(black_box(SIMPLE_PACKAGE)))
    });
}

fn bench_complex_package(c: &mut Criterion) {
    c.bench_function("sort_complex_package", |b| {
        b.iter(|| sort_package_json(black_box(COMPLEX_PACKAGE)))
    });
}

fn bench_idempotent_sort(c: &mut Criterion) {
    c.bench_function("sort_idempotent", |b| {
        let sorted = sort_package_json(COMPLEX_PACKAGE).unwrap();
        b.iter(|| sort_package_json(black_box(&sorted)))
    });
}

fn bench_fixture(c: &mut Criterion) {
    let fixture = std::fs::read_to_string("tests/fixtures/package.json")
        .expect("Failed to read fixture file");

    c.bench_function("sort_fixture", |b| {
        b.iter(|| sort_package_json(black_box(&fixture)))
    });
}

criterion_group!(
    benches,
    bench_simple_package,
    bench_complex_package,
    bench_idempotent_sort,
    bench_fixture
);
criterion_main!(benches);
