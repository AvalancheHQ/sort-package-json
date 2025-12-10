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
    "build": "tsc"
  }
}"#;

const MEDIUM_PACKAGE_JSON: &str = r#"{
  "version": "1.0.0",
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "lodash": "^4.17.21",
    "express": "^4.18.2",
    "axios": "^1.3.0",
    "typescript": "^4.9.5"
  },
  "devDependencies": {
    "jest": "^29.5.0",
    "eslint": "^8.36.0",
    "prettier": "^2.8.4",
    "webpack": "^5.76.0"
  },
  "name": "medium-package",
  "description": "A medium-sized package.json",
  "keywords": ["test", "benchmark", "example"],
  "scripts": {
    "test": "jest",
    "build": "webpack",
    "dev": "webpack serve",
    "lint": "eslint .",
    "format": "prettier --write ."
  },
  "author": {
    "name": "John Doe",
    "email": "john@example.com"
  },
  "repository": {
    "type": "git",
    "url": "https://github.com/example/repo"
  },
  "engines": {
    "node": ">=14.0.0",
    "npm": ">=6.0.0"
  }
}"#;

const LARGE_PACKAGE_JSON: &str = r#"{
  "version": "2.5.0",
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "redux": "^4.2.1",
    "react-redux": "^8.0.5",
    "lodash": "^4.17.21",
    "express": "^4.18.2",
    "axios": "^1.3.0",
    "typescript": "^4.9.5",
    "moment": "^2.29.4",
    "uuid": "^9.0.0",
    "classnames": "^2.3.2",
    "prop-types": "^15.8.1"
  },
  "devDependencies": {
    "jest": "^29.5.0",
    "eslint": "^8.36.0",
    "prettier": "^2.8.4",
    "webpack": "^5.76.0",
    "babel-loader": "^9.1.2",
    "css-loader": "^6.7.3",
    "style-loader": "^3.3.1",
    "html-webpack-plugin": "^5.5.0",
    "@testing-library/react": "^14.0.0",
    "@testing-library/jest-dom": "^5.16.5",
    "@types/react": "^18.0.28",
    "@types/react-dom": "^18.0.11",
    "@types/node": "^18.15.3"
  },
  "peerDependencies": {
    "react": ">=16.8.0"
  },
  "name": "large-package",
  "displayName": "Large Package Example",
  "description": "A large package.json with many fields",
  "keywords": ["test", "benchmark", "example", "large", "comprehensive"],
  "homepage": "https://example.com",
  "bugs": {
    "url": "https://github.com/example/repo/issues",
    "email": "bugs@example.com"
  },
  "license": "MIT",
  "author": {
    "name": "John Doe",
    "email": "john@example.com",
    "url": "https://johndoe.com"
  },
  "contributors": [
    {
      "name": "Jane Smith",
      "email": "jane@example.com"
    }
  ],
  "scripts": {
    "test": "jest",
    "test:watch": "jest --watch",
    "test:coverage": "jest --coverage",
    "build": "webpack --mode production",
    "build:dev": "webpack --mode development",
    "dev": "webpack serve --mode development",
    "lint": "eslint .",
    "lint:fix": "eslint . --fix",
    "format": "prettier --write .",
    "format:check": "prettier --check .",
    "typecheck": "tsc --noEmit"
  },
  "repository": {
    "type": "git",
    "url": "https://github.com/example/repo"
  },
  "engines": {
    "node": ">=14.0.0",
    "npm": ">=6.0.0"
  },
  "browserslist": [
    ">0.2%",
    "not dead",
    "not op_mini all"
  ],
  "files": [
    "dist",
    "src",
    "README.md",
    "LICENSE"
  ],
  "main": "dist/index.js",
  "module": "dist/index.esm.js",
  "types": "dist/index.d.ts",
  "exports": {
    ".": {
      "types": "./dist/index.d.ts",
      "import": "./dist/index.esm.js",
      "require": "./dist/index.js"
    },
    "./package.json": "./package.json"
  },
  "sideEffects": false,
  "publishConfig": {
    "access": "public",
    "registry": "https://registry.npmjs.org/"
  }
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

fn bench_idempotent_sort(c: &mut Criterion) {
    let sorted_once = sort_package_json(MEDIUM_PACKAGE_JSON).expect("First sort failed");
    
    c.bench_function("sort already sorted package.json", |b| {
        b.iter(|| sort_package_json(black_box(&sorted_once)))
    });
}

criterion_group!(
    benches,
    bench_small_package,
    bench_medium_package,
    bench_large_package,
    bench_idempotent_sort
);
criterion_main!(benches);
