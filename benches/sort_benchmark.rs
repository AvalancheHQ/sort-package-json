use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

const SIMPLE_PACKAGE_JSON: &str = r#"{
  "version": "1.0.0",
  "dependencies": {
    "lodash": "^4.17.21",
    "express": "^4.18.2"
  },
  "name": "my-package",
  "scripts": {
    "test": "jest",
    "build": "tsc"
  },
  "description": "A simple package"
}"#;

const COMPLEX_PACKAGE_JSON: &str = r#"{
  "version": "2.5.0",
  "license": "MIT",
  "devDependencies": {
    "typescript": "^5.0.0",
    "jest": "^29.0.0",
    "eslint": "^8.0.0",
    "@types/node": "^18.0.0"
  },
  "name": "@scope/complex-package",
  "scripts": {
    "test": "jest",
    "test:watch": "jest --watch",
    "build": "tsc",
    "build:prod": "tsc --build",
    "lint": "eslint .",
    "lint:fix": "eslint . --fix",
    "format": "prettier --write .",
    "prepublishOnly": "npm run build"
  },
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "lodash": "^4.17.21",
    "axios": "^1.0.0"
  },
  "keywords": ["react", "typescript", "library"],
  "author": {
    "name": "John Doe",
    "email": "john@example.com",
    "url": "https://example.com"
  },
  "repository": {
    "type": "git",
    "url": "https://github.com/example/repo"
  },
  "bugs": {
    "url": "https://github.com/example/repo/issues",
    "email": "bugs@example.com"
  },
  "homepage": "https://example.com",
  "description": "A complex package with many fields",
  "main": "dist/index.js",
  "types": "dist/index.d.ts",
  "files": ["dist", "README.md", "LICENSE"],
  "engines": {
    "node": ">=14.0.0",
    "npm": ">=6.0.0"
  },
  "peerDependencies": {
    "react": "^18.0.0"
  }
}"#;

const NESTED_OBJECTS_PACKAGE_JSON: &str = r#"{
  "name": "nested-config-package",
  "version": "1.0.0",
  "babel": {
    "presets": ["@babel/preset-env", "@babel/preset-react"],
    "plugins": ["@babel/plugin-transform-runtime"]
  },
  "jest": {
    "testEnvironment": "jsdom",
    "collectCoverage": true,
    "coverageDirectory": "coverage",
    "testMatch": ["**/__tests__/**/*.test.js"]
  },
  "eslintConfig": {
    "extends": ["eslint:recommended", "plugin:react/recommended"],
    "rules": {
      "no-console": "warn",
      "semi": ["error", "always"]
    }
  },
  "dependencies": {
    "react": "^18.2.0"
  }
}"#;

const EXPORTS_PACKAGE_JSON: &str = r#"{
  "name": "exports-package",
  "version": "1.0.0",
  "exports": {
    ".": {
      "import": "./dist/index.mjs",
      "require": "./dist/index.cjs",
      "types": "./dist/index.d.ts",
      "default": "./dist/index.js"
    },
    "./utils": {
      "import": "./dist/utils.mjs",
      "require": "./dist/utils.cjs",
      "types": "./dist/utils.d.ts"
    },
    "./package.json": "./package.json"
  }
}"#;

fn bench_simple_package(c: &mut Criterion) {
    c.bench_function("sort simple package.json", |b| {
        b.iter(|| sort_package_json(black_box(SIMPLE_PACKAGE_JSON)))
    });
}

fn bench_complex_package(c: &mut Criterion) {
    c.bench_function("sort complex package.json", |b| {
        b.iter(|| sort_package_json(black_box(COMPLEX_PACKAGE_JSON)))
    });
}

fn bench_nested_objects(c: &mut Criterion) {
    c.bench_function("sort package.json with nested config objects", |b| {
        b.iter(|| sort_package_json(black_box(NESTED_OBJECTS_PACKAGE_JSON)))
    });
}

fn bench_exports(c: &mut Criterion) {
    c.bench_function("sort package.json with exports field", |b| {
        b.iter(|| sort_package_json(black_box(EXPORTS_PACKAGE_JSON)))
    });
}

criterion_group!(
    benches,
    bench_simple_package,
    bench_complex_package,
    bench_nested_objects,
    bench_exports
);
criterion_main!(benches);
