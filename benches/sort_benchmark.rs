use codspeed_criterion_compat::{Criterion, criterion_group, criterion_main};
use sort_package_json::sort_package_json;
use std::fs;

fn bench_sort_small_package_json(c: &mut Criterion) {
    let input =
        fs::read_to_string("tests/fixtures/package.json").expect("Failed to read fixture file");

    c.bench_function("sort_small_package_json", |b| {
        b.iter(|| {
            sort_package_json(&input).expect("Failed to sort package.json");
        });
    });
}

fn bench_sort_idempotent(c: &mut Criterion) {
    let input =
        fs::read_to_string("tests/fixtures/package.json").expect("Failed to read fixture file");
    let sorted_once = sort_package_json(&input).expect("Failed to sort package.json");

    c.bench_function("sort_idempotent", |b| {
        b.iter(|| {
            sort_package_json(&sorted_once).expect("Failed to sort package.json");
        });
    });
}

fn bench_sort_large_package_json(c: &mut Criterion) {
    // Create a larger package.json for benchmarking by duplicating some fields
    let large_input = r#"{
  "name": "large-test-package",
  "version": "1.0.0",
  "description": "A large test package for performance testing",
  "keywords": ["test", "sorting", "json", "performance", "benchmark", "large", "example", "demo", "testing", "rust"],
  "homepage": "https://github.com/test/large-test-package#readme",
  "bugs": {
    "email": "bugs@example.com",
    "url": "https://github.com/test/large-test-package/issues"
  },
  "repository": {
    "url": "https://github.com/test/large-test-package",
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
    },
    "./utils": {
      "import": "./dist/utils.esm.js",
      "default": "./dist/utils.js",
      "types": "./dist/utils.d.ts"
    },
    "./helpers": {
      "import": "./dist/helpers.esm.js",
      "default": "./dist/helpers.js",
      "types": "./dist/helpers.d.ts"
    }
  },
  "main": "./dist/index.js",
  "module": "./dist/index.esm.js",
  "types": "./dist/index.d.ts",
  "scripts": {
    "test": "jest",
    "test:unit": "jest --testPathPattern=unit",
    "test:integration": "jest --testPathPattern=integration",
    "test:e2e": "jest --testPathPattern=e2e",
    "test:watch": "jest --watch",
    "test:coverage": "jest --coverage",
    "build": "webpack",
    "build:dev": "webpack --mode development",
    "build:prod": "webpack --mode production",
    "lint": "eslint .",
    "lint:fix": "eslint . --fix",
    "format": "prettier --write .",
    "dev": "webpack serve",
    "start": "node dist/index.js"
  },
  "dependencies": {
    "react": "^18.0.0",
    "react-dom": "^18.0.0",
    "axios": "^1.0.0",
    "lodash": "^4.17.21",
    "express": "^4.18.0",
    "dotenv": "^16.0.0",
    "uuid": "^9.0.0",
    "moment": "^2.29.0"
  },
  "devDependencies": {
    "webpack": "^5.0.0",
    "webpack-cli": "^5.0.0",
    "webpack-dev-server": "^4.0.0",
    "jest": "^29.0.0",
    "typescript": "^5.0.0",
    "eslint": "^8.0.0",
    "prettier": "^3.0.0",
    "@types/node": "^20.0.0",
    "@types/react": "^18.0.0"
  },
  "engines": {
    "npm": ">=8.0.0",
    "node": ">=18.0.0"
  },
  "babel": {
    "presets": ["@babel/preset-env", "@babel/preset-react", "@babel/preset-typescript"],
    "plugins": ["@babel/plugin-proposal-class-properties", "@babel/plugin-transform-runtime"]
  },
  "eslintConfig": {
    "extends": ["eslint:recommended", "plugin:react/recommended"],
    "rules": {
      "no-console": "warn",
      "no-unused-vars": "error"
    }
  },
  "jest": {
    "testEnvironment": "node",
    "coverageThreshold": {
      "global": {
        "branches": 80,
        "functions": 80,
        "lines": 80,
        "statements": 80
      }
    }
  },
  "publishConfig": {
    "access": "public"
  }
}"#;

    c.bench_function("sort_large_package_json", |b| {
        b.iter(|| {
            sort_package_json(large_input).expect("Failed to sort package.json");
        });
    });
}

criterion_group!(
    benches,
    bench_sort_small_package_json,
    bench_sort_idempotent,
    bench_sort_large_package_json
);
criterion_main!(benches);
