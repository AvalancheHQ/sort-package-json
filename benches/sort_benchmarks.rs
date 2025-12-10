use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

const SMALL_PACKAGE_JSON: &str = r#"{
  "name": "simple-package",
  "version": "1.0.0",
  "dependencies": {
    "react": "^18.0.0",
    "axios": "^1.0.0"
  },
  "devDependencies": {
    "jest": "^29.0.0"
  }
}"#;

const MEDIUM_PACKAGE_JSON: &str = r#"{
  "name": "test-package",
  "version": "1.0.0",
  "description": "A test package for sorting",
  "keywords": ["test", "sorting", "json", "test", "example"],
  "homepage": "https://github.com/test/test-package#readme",
  "bugs": {
    "email": "bugs@example.com",
    "url": "https://github.com/test/test-package/issues"
  },
  "repository": {
    "url": "https://github.com/test/test-package",
    "type": "git"
  },
  "license": "MIT",
  "author": {
    "url": "https://example.com",
    "email": "author@example.com",
    "name": "Test Author"
  },
  "exports": {
    ".": {
      "default": "./dist/index.js",
      "require": "./dist/index.cjs",
      "types": "./dist/index.d.ts",
      "import": "./dist/index.esm.js"
    }
  },
  "main": "./dist/index.js",
  "scripts": {
    "test": "jest",
    "build": "webpack",
    "lint": "eslint ."
  },
  "dependencies": {
    "react": "^18.0.0",
    "axios": "^1.0.0",
    "lodash": "^4.17.21"
  },
  "devDependencies": {
    "webpack": "^5.0.0",
    "jest": "^29.0.0",
    "typescript": "^5.0.0"
  }
}"#;

fn bench_sort_small_package(c: &mut Criterion) {
    c.bench_function("sort_small_package", |b| {
        b.iter(|| {
            sort_package_json(SMALL_PACKAGE_JSON).expect("Failed to sort small package.json")
        });
    });
}

fn bench_sort_medium_package(c: &mut Criterion) {
    c.bench_function("sort_medium_package", |b| {
        b.iter(|| {
            sort_package_json(MEDIUM_PACKAGE_JSON).expect("Failed to sort medium package.json")
        });
    });
}

fn bench_sort_with_complex_exports(c: &mut Criterion) {
    let json_with_exports = r#"{
      "name": "exports-test",
      "exports": {
        "./package.json": "./package.json",
        ".": {
          "default": "./dist/index.js",
          "require": "./dist/index.cjs",
          "types": "./dist/index.d.ts",
          "import": "./dist/index.esm.js"
        },
        "./utils": {
          "import": "./dist/utils.esm.js",
          "default": "./dist/utils.js",
          "types": "./dist/utils.d.ts"
        },
        "./helpers": {
          "types": "./dist/helpers.d.ts",
          "import": "./dist/helpers.esm.js",
          "default": "./dist/helpers.js"
        }
      }
    }"#;

    c.bench_function("sort_complex_exports", |b| {
        b.iter(|| {
            sort_package_json(json_with_exports).expect("Failed to sort exports")
        });
    });
}

fn bench_sort_with_arrays(c: &mut Criterion) {
    let json_with_arrays = r#"{
      "name": "arrays-test",
      "keywords": ["test", "sorting", "json", "test", "example", "benchmark", "test"],
      "files": ["src", "dist", "README.md", "dist", "LICENSE", "src"],
      "dependencies": {
        "react": "^18.0.0",
        "axios": "^1.0.0",
        "lodash": "^4.17.21"
      }
    }"#;

    c.bench_function("sort_with_arrays", |b| {
        b.iter(|| {
            sort_package_json(json_with_arrays).expect("Failed to sort arrays")
        });
    });
}

fn bench_sort_idempotency(c: &mut Criterion) {
    let sorted = sort_package_json(MEDIUM_PACKAGE_JSON).expect("Failed to sort");
    
    c.bench_function("sort_idempotency", |b| {
        b.iter(|| {
            sort_package_json(&sorted).expect("Failed to sort already sorted package.json")
        });
    });
}

criterion_group!(
    benches,
    bench_sort_small_package,
    bench_sort_medium_package,
    bench_sort_with_complex_exports,
    bench_sort_with_arrays,
    bench_sort_idempotency
);
criterion_main!(benches);
