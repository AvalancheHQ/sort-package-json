use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

fn benchmark_simple_package(c: &mut Criterion) {
    let simple_json = r#"{
  "name": "my-package",
  "version": "1.0.0",
  "description": "A test package",
  "main": "index.js",
  "scripts": {
    "test": "jest",
    "build": "webpack"
  },
  "keywords": ["test", "example"],
  "author": "Test Author",
  "license": "MIT",
  "dependencies": {
    "express": "^4.18.0",
    "lodash": "^4.17.21"
  },
  "devDependencies": {
    "jest": "^29.0.0",
    "webpack": "^5.0.0"
  }
}"#;

    c.bench_function("sort simple package.json", |b| {
        b.iter(|| sort_package_json(black_box(simple_json)))
    });
}

fn benchmark_complex_package(c: &mut Criterion) {
    let complex_json = r#"{
  "workspaces": ["packages/*"],
  "scripts": {
    "pretest": "npm run lint",
    "test": "jest",
    "posttest": "npm run coverage",
    "prebuild": "npm run clean",
    "build": "tsc",
    "postbuild": "npm run copy-files",
    "lint": "eslint src",
    "format": "prettier --write .",
    "clean": "rimraf dist"
  },
  "devDependencies": {
    "eslint": "^8.0.0",
    "jest": "^29.0.0",
    "prettier": "^3.0.0",
    "rimraf": "^5.0.0",
    "typescript": "^5.0.0",
    "webpack": "^5.0.0",
    "@types/node": "^20.0.0"
  },
  "dependencies": {
    "express": "^4.18.0",
    "lodash": "^4.17.21",
    "axios": "^1.0.0",
    "dotenv": "^16.0.0"
  },
  "peerDependencies": {
    "react": "^18.0.0"
  },
  "optionalDependencies": {
    "fsevents": "^2.3.0"
  },
  "name": "complex-package",
  "version": "2.5.0",
  "description": "A complex package with many dependencies",
  "keywords": ["test", "complex", "example", "benchmark"],
  "author": {
    "name": "Test Author",
    "email": "test@example.com",
    "url": "https://example.com"
  },
  "license": "MIT",
  "repository": {
    "type": "git",
    "url": "https://github.com/test/complex-package.git"
  },
  "bugs": {
    "url": "https://github.com/test/complex-package/issues",
    "email": "bugs@example.com"
  },
  "homepage": "https://example.com",
  "engines": {
    "node": ">=18.0.0",
    "npm": ">=9.0.0"
  },
  "exports": {
    ".": {
      "types": "./dist/index.d.ts",
      "import": "./dist/index.mjs",
      "require": "./dist/index.js",
      "default": "./dist/index.js"
    },
    "./package.json": "./package.json"
  },
  "files": ["dist", "README.md", "LICENSE"],
  "type": "module",
  "main": "./dist/index.js",
  "module": "./dist/index.mjs",
  "types": "./dist/index.d.ts"
}"#;

    c.bench_function("sort complex package.json", |b| {
        b.iter(|| sort_package_json(black_box(complex_json)))
    });
}

fn benchmark_with_nested_configs(c: &mut Criterion) {
    let nested_json = r#"{
  "name": "nested-configs",
  "version": "1.0.0",
  "jest": {
    "testEnvironment": "node",
    "coverageDirectory": "coverage",
    "collectCoverageFrom": ["src/**/*.ts"],
    "testMatch": ["**/*.test.ts"],
    "transform": {
      "^.+\\.ts$": "ts-jest"
    }
  },
  "eslintConfig": {
    "extends": ["eslint:recommended"],
    "rules": {
      "no-console": "warn",
      "semi": ["error", "always"]
    },
    "env": {
      "node": true,
      "es6": true
    }
  },
  "prettier": {
    "semi": true,
    "singleQuote": true,
    "tabWidth": 2,
    "trailingComma": "all"
  },
  "babel": {
    "presets": ["@babel/preset-env", "@babel/preset-typescript"],
    "plugins": ["@babel/plugin-transform-runtime"]
  }
}"#;

    c.bench_function("sort package.json with nested configs", |b| {
        b.iter(|| sort_package_json(black_box(nested_json)))
    });
}

criterion_group!(
    benches,
    benchmark_simple_package,
    benchmark_complex_package,
    benchmark_with_nested_configs
);
criterion_main!(benches);
