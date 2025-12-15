use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

const SMALL_PACKAGE_JSON: &str = r#"{
  "version": "1.0.0",
  "dependencies": {
    "lodash": "^4.17.21",
    "express": "^4.18.2"
  },
  "name": "my-package",
  "scripts": {
    "test": "jest",
    "build": "webpack"
  },
  "description": "A test package"
}"#;

const MEDIUM_PACKAGE_JSON: &str = r#"{
  "version": "2.1.0",
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "axios": "^1.4.0",
    "lodash": "^4.17.21",
    "moment": "^2.29.4"
  },
  "name": "medium-package",
  "devDependencies": {
    "webpack": "^5.88.0",
    "babel-loader": "^9.1.2",
    "@types/react": "^18.2.14",
    "eslint": "^8.44.0",
    "prettier": "^2.8.8"
  },
  "scripts": {
    "start": "webpack serve",
    "build": "webpack --mode production",
    "test": "jest",
    "lint": "eslint src",
    "format": "prettier --write src"
  },
  "keywords": ["react", "webpack", "babel"],
  "author": {
    "name": "John Doe",
    "email": "john@example.com"
  },
  "license": "MIT",
  "repository": {
    "type": "git",
    "url": "https://github.com/example/repo.git"
  },
  "bugs": {
    "url": "https://github.com/example/repo/issues"
  },
  "homepage": "https://example.com",
  "engines": {
    "node": ">=16.0.0"
  }
}"#;

const LARGE_PACKAGE_JSON: &str = r#"{
  "workspaces": ["packages/*"],
  "private": true,
  "scripts": {
    "build": "lerna run build",
    "test": "lerna run test",
    "lint": "eslint .",
    "format": "prettier --write .",
    "clean": "lerna clean",
    "bootstrap": "lerna bootstrap",
    "publish": "lerna publish",
    "version": "lerna version",
    "precommit": "lint-staged",
    "prepare": "husky install"
  },
  "devDependencies": {
    "lerna": "^7.0.0",
    "@typescript-eslint/eslint-plugin": "^5.60.0",
    "@typescript-eslint/parser": "^5.60.0",
    "eslint": "^8.44.0",
    "eslint-config-prettier": "^8.8.0",
    "eslint-plugin-react": "^7.32.2",
    "prettier": "^2.8.8",
    "husky": "^8.0.3",
    "lint-staged": "^13.2.3",
    "typescript": "^5.1.6"
  },
  "lint-staged": {
    "*.{js,jsx,ts,tsx}": ["eslint --fix", "prettier --write"],
    "*.{json,md,yml}": ["prettier --write"]
  },
  "husky": {
    "hooks": {
      "pre-commit": "lint-staged"
    }
  },
  "engines": {
    "node": ">=18.0.0",
    "npm": ">=9.0.0"
  },
  "version": "1.0.0",
  "name": "monorepo-root",
  "repository": {
    "type": "git",
    "url": "https://github.com/example/monorepo.git"
  },
  "author": "Engineering Team",
  "license": "MIT"
}"#;

fn bench_small_package(c: &mut Criterion) {
    c.bench_function("sort small package.json", |b| {
        b.iter(|| sort_package_json(black_box(SMALL_PACKAGE_JSON)))
    });
}

fn bench_medium_package(c: &mut Criterion) {
    c.bench_function("sort medium package.json", |b| {
        b.iter(|| sort_package_json(black_box(MEDIUM_PACKAGE_JSON)))
    });
}

fn bench_large_package(c: &mut Criterion) {
    c.bench_function("sort large package.json", |b| {
        b.iter(|| sort_package_json(black_box(LARGE_PACKAGE_JSON)))
    });
}

fn bench_idempotency(c: &mut Criterion) {
    let sorted_once = sort_package_json(MEDIUM_PACKAGE_JSON).unwrap();
    c.bench_function("sort already sorted package.json", |b| {
        b.iter(|| sort_package_json(black_box(&sorted_once)))
    });
}

criterion_group!(
    benches,
    bench_small_package,
    bench_medium_package,
    bench_large_package,
    bench_idempotency
);
criterion_main!(benches);
