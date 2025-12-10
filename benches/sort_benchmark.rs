use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

// Sample package.json fixtures for benchmarking
const SMALL_PACKAGE_JSON: &str = r#"{
  "name": "small-package",
  "version": "1.0.0",
  "description": "A small test package",
  "main": "index.js",
  "scripts": {
    "test": "jest"
  },
  "dependencies": {
    "react": "^18.0.0"
  }
}"#;

const MEDIUM_PACKAGE_JSON: &str = r#"{
  "name": "medium-package",
  "version": "1.0.0",
  "description": "A medium test package",
  "keywords": ["test", "sorting", "json"],
  "homepage": "https://github.com/test/test-package",
  "bugs": {
    "url": "https://github.com/test/test-package/issues"
  },
  "license": "MIT",
  "main": "./dist/index.js",
  "module": "./dist/index.esm.js",
  "scripts": {
    "test": "jest",
    "build": "webpack",
    "lint": "eslint ."
  },
  "dependencies": {
    "react": "^18.0.0",
    "axios": "^1.0.0"
  },
  "devDependencies": {
    "webpack": "^5.0.0",
    "jest": "^29.0.0"
  }
}"#;

const LARGE_PACKAGE_JSON: &str = r#"{
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
  "maintainers": [
    {
      "url": "https://maintainer1.com",
      "name": "Maintainer One",
      "email": "maintainer1@example.com"
    },
    {
      "email": "maintainer2@example.com",
      "url": "https://maintainer2.com",
      "name": "Maintainer Two"
    }
  ],
  "exports": {
    ".": {
      "default": "./dist/index.js",
      "require": "./dist/index.cjs",
      "types": "./dist/index.d.ts",
      "import": "./dist/index.esm.js"
    },
    "./package.json": "./package.json",
    "./utils": {
      "import": "./dist/utils.esm.js",
      "default": "./dist/utils.js",
      "types": "./dist/utils.d.ts"
    }
  },
  "main": "./dist/index.js",
  "module": "./dist/index.esm.js",
  "types": "./dist/index.d.ts",
  "files": ["src", "dist", "README.md", "dist", "LICENSE"],
  "scripts": {
    "test": "jest",
    "posttest": "echo 'Tests complete'",
    "build": "webpack",
    "lint": "eslint .",
    "pretest": "echo 'Starting tests'",
    "dev": "webpack serve"
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
  },
  "engines": {
    "npm": ">=8.0.0",
    "node": ">=18.0.0"
  },
  "babel": {
    "presets": ["@babel/preset-env", "@babel/preset-react"],
    "plugins": ["@babel/plugin-proposal-class-properties"]
  },
  "publishConfig": {
    "access": "public"
  },
  "customField": "this is a custom unknown field",
  "_custom": "another private field",
  "_id": "test-package@1.0.0"
}"#;

fn bench_small_package(c: &mut Criterion) {
    c.bench_function("sort_small_package_json", |b| {
        b.iter(|| sort_package_json(SMALL_PACKAGE_JSON))
    });
}

fn bench_medium_package(c: &mut Criterion) {
    c.bench_function("sort_medium_package_json", |b| {
        b.iter(|| sort_package_json(MEDIUM_PACKAGE_JSON))
    });
}

fn bench_large_package(c: &mut Criterion) {
    c.bench_function("sort_large_package_json", |b| {
        b.iter(|| sort_package_json(LARGE_PACKAGE_JSON))
    });
}

fn bench_idempotency(c: &mut Criterion) {
    let sorted_once = sort_package_json(LARGE_PACKAGE_JSON).unwrap();
    c.bench_function("sort_idempotency", |b| {
        b.iter(|| sort_package_json(&sorted_once))
    });
}

fn bench_multiple_fields(c: &mut Criterion) {
    // Test with maximum field variety
    let complex_json = r#"{
        "version": "1.0.0",
        "name": "complex",
        "_private1": "value",
        "customZ": "last custom",
        "scripts": {"test": "jest", "build": "webpack"},
        "dependencies": {"z": "1.0", "a": "2.0", "m": "3.0"},
        "keywords": ["z", "a", "m", "a"],
        "_private2": "value2",
        "customA": "first custom",
        "author": {"email": "test@test.com", "name": "Test", "url": "http://test.com"},
        "exports": {
            "./utils": {"types": "./utils.d.ts", "import": "./utils.js"},
            ".": {"types": "./index.d.ts", "default": "./index.js"}
        }
    }"#;
    
    c.bench_function("sort_complex_fields", |b| {
        b.iter(|| sort_package_json(complex_json))
    });
}

criterion_group!(
    benches,
    bench_small_package,
    bench_medium_package,
    bench_large_package,
    bench_idempotency,
    bench_multiple_fields
);

criterion_main!(benches);
