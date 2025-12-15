use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

const SIMPLE_PACKAGE_JSON: &str = r#"{
  "name": "test-package",
  "dependencies": {
    "react": "^18.0.0",
    "lodash": "^4.17.21"
  },
  "version": "1.0.0",
  "scripts": {
    "test": "jest",
    "build": "webpack"
  },
  "devDependencies": {
    "webpack": "^5.0.0",
    "jest": "^29.0.0"
  }
}"#;

const COMPLEX_PACKAGE_JSON: &str = r#"{
  "name": "complex-package",
  "version": "1.2.3",
  "description": "A complex package.json for benchmarking",
  "main": "./index.js",
  "scripts": {
    "test": "jest",
    "build": "webpack --mode production",
    "dev": "webpack serve",
    "lint": "eslint .",
    "format": "prettier --write ."
  },
  "keywords": ["performance", "benchmark", "test"],
  "author": {
    "name": "Test Author",
    "email": "test@example.com"
  },
  "license": "MIT",
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "lodash": "^4.17.21",
    "axios": "^1.4.0"
  },
  "devDependencies": {
    "webpack": "^5.88.0",
    "webpack-cli": "^5.1.4",
    "jest": "^29.6.0",
    "eslint": "^8.44.0",
    "prettier": "^3.0.0"
  },
  "peerDependencies": {
    "react": ">=18.0.0"
  },
  "engines": {
    "node": ">=16.0.0",
    "npm": ">=8.0.0"
  },
  "repository": {
    "type": "git",
    "url": "https://github.com/example/repo"
  },
  "bugs": {
    "url": "https://github.com/example/repo/issues"
  },
  "homepage": "https://example.com"
}"#;

const EXPORTS_PACKAGE_JSON: &str = r#"{
  "name": "exports-package",
  "version": "1.0.0",
  "exports": {
    "./package.json": "./package.json",
    ".": {
      "import": "./index.mjs",
      "require": "./index.js",
      "types": "./index.d.ts",
      "default": "./index.js"
    },
    "./utils": {
      "import": "./utils.mjs",
      "require": "./utils.js",
      "types": "./utils.d.ts"
    }
  },
  "dependencies": {
    "zod": "^3.0.0",
    "react": "^18.0.0"
  }
}"#;

fn bench_simple_sort(c: &mut Criterion) {
    c.bench_function("sort_simple_package_json", |b| {
        b.iter(|| sort_package_json(SIMPLE_PACKAGE_JSON))
    });
}

fn bench_complex_sort(c: &mut Criterion) {
    c.bench_function("sort_complex_package_json", |b| {
        b.iter(|| sort_package_json(COMPLEX_PACKAGE_JSON))
    });
}

fn bench_exports_sort(c: &mut Criterion) {
    c.bench_function("sort_exports_package_json", |b| {
        b.iter(|| sort_package_json(EXPORTS_PACKAGE_JSON))
    });
}

fn bench_repeated_sort(c: &mut Criterion) {
    c.bench_function("sort_package_json_repeated", |b| {
        b.iter(|| {
            for _ in 0..10 {
                let _ = sort_package_json(SIMPLE_PACKAGE_JSON);
            }
        })
    });
}

criterion_group!(
    benches,
    bench_simple_sort,
    bench_complex_sort,
    bench_exports_sort,
    bench_repeated_sort
);
criterion_main!(benches);
