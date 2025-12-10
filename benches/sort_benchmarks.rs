use codspeed_criterion_compat::{criterion_group, criterion_main, BatchSize, Criterion};
use sort_package_json::sort_package_json;

const SIMPLE_PACKAGE_JSON: &str = r#"{
  "name": "simple-package",
  "version": "1.0.0",
  "description": "A simple package",
  "main": "index.js",
  "scripts": {
    "test": "jest",
    "build": "webpack"
  },
  "dependencies": {
    "react": "^18.0.0"
  }
}"#;

const COMPLEX_PACKAGE_JSON: &str = r#"{
  "name": "complex-package",
  "version": "2.5.0",
  "description": "A complex package with many fields",
  "keywords": ["test", "sorting", "json", "test", "example", "benchmark"],
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
    "lodash": "^4.17.21",
    "vue": "^3.0.0",
    "express": "^4.18.0"
  },
  "devDependencies": {
    "webpack": "^5.0.0",
    "jest": "^29.0.0",
    "typescript": "^5.0.0",
    "eslint": "^8.0.0",
    "prettier": "^3.0.0"
  },
  "peerDependencies": {
    "react": ">=16.0.0"
  },
  "engines": {
    "npm": ">=8.0.0",
    "node": ">=18.0.0"
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
  },
  "prettier": {
    "semi": true,
    "singleQuote": true
  },
  "publishConfig": {
    "access": "public",
    "registry": "https://registry.npmjs.org/"
  },
  "customField": "this is a custom unknown field",
  "anotherCustom": "value",
  "_custom": "another private field",
  "_id": "test-package@1.0.0"
}"#;

const MONOREPO_PACKAGE_JSON: &str = r#"{
  "name": "monorepo-root",
  "version": "1.0.0",
  "private": true,
  "workspaces": [
    "packages/*",
    "apps/*"
  ],
  "scripts": {
    "test": "turbo run test",
    "build": "turbo run build",
    "lint": "turbo run lint",
    "dev": "turbo run dev --parallel"
  },
  "devDependencies": {
    "turbo": "^1.10.0",
    "typescript": "^5.0.0",
    "prettier": "^3.0.0",
    "eslint": "^8.0.0"
  },
  "packageManager": "pnpm@8.6.0",
  "engines": {
    "node": ">=18.0.0",
    "pnpm": ">=8.0.0"
  }
}"#;

fn bench_simple_package(c: &mut Criterion) {
    c.bench_function("sort_simple_package", |b| {
        b.iter_batched(
            || SIMPLE_PACKAGE_JSON.to_string(),
            |input| sort_package_json(&input).unwrap(),
            BatchSize::SmallInput,
        )
    });
}

fn bench_complex_package(c: &mut Criterion) {
    c.bench_function("sort_complex_package", |b| {
        b.iter_batched(
            || COMPLEX_PACKAGE_JSON.to_string(),
            |input| sort_package_json(&input).unwrap(),
            BatchSize::SmallInput,
        )
    });
}

fn bench_monorepo_package(c: &mut Criterion) {
    c.bench_function("sort_monorepo_package", |b| {
        b.iter_batched(
            || MONOREPO_PACKAGE_JSON.to_string(),
            |input| sort_package_json(&input).unwrap(),
            BatchSize::SmallInput,
        )
    });
}

fn bench_fixture_package(c: &mut Criterion) {
    let fixture_content =
        std::fs::read_to_string("tests/fixtures/package.json").expect("Failed to read fixture");

    c.bench_function("sort_fixture_package", |b| {
        b.iter_batched(
            || fixture_content.clone(),
            |input| sort_package_json(&input).unwrap(),
            BatchSize::SmallInput,
        )
    });
}

fn bench_idempotency(c: &mut Criterion) {
    let sorted_once = sort_package_json(COMPLEX_PACKAGE_JSON).unwrap();

    c.bench_function("sort_idempotent_package", |b| {
        b.iter_batched(
            || sorted_once.clone(),
            |input| sort_package_json(&input).unwrap(),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    benches,
    bench_simple_package,
    bench_complex_package,
    bench_monorepo_package,
    bench_fixture_package,
    bench_idempotency
);
criterion_main!(benches);
