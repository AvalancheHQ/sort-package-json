use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

const SMALL_PACKAGE_JSON: &str = r#"{
  "version": "1.0.0",
  "dependencies": {
    "lodash": "^4.17.21",
    "react": "^18.0.0"
  },
  "name": "test-package",
  "scripts": {
    "test": "jest",
    "build": "webpack"
  },
  "description": "A test package"
}"#;

const MEDIUM_PACKAGE_JSON: &str = r#"{
  "version": "2.0.0",
  "dependencies": {
    "express": "^4.18.0",
    "axios": "^1.0.0",
    "lodash": "^4.17.21",
    "react": "^18.0.0",
    "react-dom": "^18.0.0"
  },
  "devDependencies": {
    "webpack": "^5.0.0",
    "babel-loader": "^9.0.0",
    "eslint": "^8.0.0",
    "jest": "^29.0.0",
    "typescript": "^5.0.0"
  },
  "name": "medium-package",
  "scripts": {
    "test": "jest",
    "build": "webpack",
    "lint": "eslint src",
    "dev": "webpack-dev-server",
    "start": "node server.js"
  },
  "keywords": ["test", "benchmark", "package"],
  "author": {
    "name": "Test Author",
    "email": "test@example.com"
  },
  "license": "MIT",
  "repository": {
    "type": "git",
    "url": "https://github.com/test/repo"
  },
  "description": "A medium-sized test package"
}"#;

const LARGE_PACKAGE_JSON: &str = r#"{
  "version": "3.0.0",
  "dependencies": {
    "express": "^4.18.0",
    "axios": "^1.0.0",
    "lodash": "^4.17.21",
    "react": "^18.0.0",
    "react-dom": "^18.0.0",
    "redux": "^4.2.0",
    "react-redux": "^8.0.0",
    "next": "^13.0.0",
    "styled-components": "^6.0.0",
    "graphql": "^16.0.0"
  },
  "devDependencies": {
    "webpack": "^5.0.0",
    "babel-loader": "^9.0.0",
    "eslint": "^8.0.0",
    "jest": "^29.0.0",
    "typescript": "^5.0.0",
    "@types/react": "^18.0.0",
    "@types/node": "^20.0.0",
    "prettier": "^3.0.0",
    "husky": "^8.0.0",
    "lint-staged": "^13.0.0"
  },
  "peerDependencies": {
    "react": ">=18.0.0"
  },
  "optionalDependencies": {
    "fsevents": "^2.3.0"
  },
  "name": "large-package",
  "scripts": {
    "test": "jest",
    "test:watch": "jest --watch",
    "test:coverage": "jest --coverage",
    "build": "webpack --mode production",
    "build:dev": "webpack --mode development",
    "lint": "eslint src",
    "lint:fix": "eslint src --fix",
    "format": "prettier --write src",
    "dev": "webpack-dev-server",
    "start": "node server.js",
    "prepare": "husky install",
    "typecheck": "tsc --noEmit"
  },
  "keywords": ["test", "benchmark", "package", "large", "example"],
  "author": {
    "name": "Test Author",
    "email": "test@example.com",
    "url": "https://example.com"
  },
  "maintainers": [
    {
      "name": "Maintainer One",
      "email": "m1@example.com"
    },
    {
      "name": "Maintainer Two",
      "email": "m2@example.com"
    }
  ],
  "license": "MIT",
  "repository": {
    "type": "git",
    "url": "https://github.com/test/large-repo"
  },
  "bugs": {
    "url": "https://github.com/test/large-repo/issues"
  },
  "homepage": "https://example.com",
  "engines": {
    "node": ">=18.0.0",
    "npm": ">=9.0.0"
  },
  "browserslist": [
    "> 1%",
    "last 2 versions"
  ],
  "eslintConfig": {
    "extends": ["eslint:recommended"],
    "rules": {
      "no-console": "warn"
    }
  },
  "jest": {
    "testEnvironment": "jsdom",
    "collectCoverage": true
  },
  "prettier": {
    "semi": true,
    "singleQuote": true
  },
  "description": "A large test package with many fields"
}"#;

fn bench_sort_small(c: &mut Criterion) {
    c.bench_function("sort_small_package_json", |b| {
        b.iter(|| sort_package_json(black_box(SMALL_PACKAGE_JSON)))
    });
}

fn bench_sort_medium(c: &mut Criterion) {
    c.bench_function("sort_medium_package_json", |b| {
        b.iter(|| sort_package_json(black_box(MEDIUM_PACKAGE_JSON)))
    });
}

fn bench_sort_large(c: &mut Criterion) {
    c.bench_function("sort_large_package_json", |b| {
        b.iter(|| sort_package_json(black_box(LARGE_PACKAGE_JSON)))
    });
}

criterion_group!(benches, bench_sort_small, bench_sort_medium, bench_sort_large);
criterion_main!(benches);
