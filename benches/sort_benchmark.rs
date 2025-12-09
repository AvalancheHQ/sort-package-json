use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

const SMALL_PACKAGE: &str = r#"{
  "name": "small-package",
  "version": "1.0.0",
  "dependencies": {
    "react": "^18.0.0"
  }
}"#;

const MEDIUM_PACKAGE: &str = include_str!("../tests/fixtures/package.json");

const LARGE_PACKAGE: &str = r#"{
  "name": "large-package",
  "version": "1.0.0",
  "description": "A large package with many fields",
  "keywords": ["test", "large", "benchmark", "performance", "sorting", "json", "package"],
  "homepage": "https://github.com/test/large-package#readme",
  "bugs": {
    "url": "https://github.com/test/large-package/issues",
    "email": "bugs@example.com"
  },
  "repository": {
    "type": "git",
    "url": "https://github.com/test/large-package"
  },
  "license": "MIT",
  "author": {
    "name": "Test Author",
    "email": "author@example.com",
    "url": "https://example.com"
  },
  "main": "./dist/index.js",
  "module": "./dist/index.esm.js",
  "types": "./dist/index.d.ts",
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
  "files": ["dist", "src", "README.md", "LICENSE"],
  "scripts": {
    "build": "webpack",
    "test": "jest",
    "lint": "eslint .",
    "format": "prettier --write .",
    "dev": "webpack serve",
    "pretest": "echo 'Starting tests'",
    "posttest": "echo 'Tests complete'",
    "prebuild": "npm run clean",
    "postbuild": "npm run test",
    "clean": "rimraf dist"
  },
  "dependencies": {
    "react": "^18.0.0",
    "react-dom": "^18.0.0",
    "axios": "^1.0.0",
    "lodash": "^4.17.21",
    "date-fns": "^2.29.0",
    "classnames": "^2.3.1"
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
  "jest": {
    "testEnvironment": "jsdom",
    "coverageDirectory": "coverage",
    "collectCoverageFrom": ["src/**/*.{js,jsx,ts,tsx}"]
  },
  "eslintConfig": {
    "extends": ["eslint:recommended", "plugin:react/recommended"],
    "rules": {
      "no-console": "warn"
    }
  },
  "prettier": {
    "semi": true,
    "singleQuote": true,
    "trailingComma": "es5"
  },
  "publishConfig": {
    "access": "public"
  },
  "customField": "this is a custom unknown field",
  "anotherCustom": "another custom field",
  "_custom": "private field",
  "_id": "large-package@1.0.0"
}"#;

fn bench_sort_small(c: &mut Criterion) {
    c.bench_function("sort_small_package", |b| {
        b.iter(|| sort_package_json(SMALL_PACKAGE).unwrap());
    });
}

fn bench_sort_medium(c: &mut Criterion) {
    c.bench_function("sort_medium_package", |b| {
        b.iter(|| sort_package_json(MEDIUM_PACKAGE).unwrap());
    });
}

fn bench_sort_large(c: &mut Criterion) {
    c.bench_function("sort_large_package", |b| {
        b.iter(|| sort_package_json(LARGE_PACKAGE).unwrap());
    });
}

fn bench_sort_idempotent(c: &mut Criterion) {
    let sorted = sort_package_json(MEDIUM_PACKAGE).unwrap();
    c.bench_function("sort_already_sorted", |b| {
        b.iter(|| sort_package_json(&sorted).unwrap());
    });
}

criterion_group!(
    benches,
    bench_sort_small,
    bench_sort_medium,
    bench_sort_large,
    bench_sort_idempotent
);
criterion_main!(benches);
