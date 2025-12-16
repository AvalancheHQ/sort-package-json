use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use sort_package_json::sort_package_json;

const SMALL_PACKAGE_JSON: &str = r#"{
  "version": "1.0.0",
  "dependencies": {
    "lodash": "^4.17.21",
    "express": "^4.18.0"
  },
  "name": "my-package",
  "scripts": {
    "test": "jest",
    "build": "tsc"
  }
}"#;

const MEDIUM_PACKAGE_JSON: &str = r#"{
  "version": "2.5.0",
  "description": "A medium-sized package.json for benchmarking",
  "main": "dist/index.js",
  "types": "dist/index.d.ts",
  "scripts": {
    "test": "jest --coverage",
    "build": "tsc",
    "lint": "eslint src/**/*.ts",
    "format": "prettier --write .",
    "prepublishOnly": "npm run build",
    "pretest": "npm run lint"
  },
  "name": "@org/medium-package",
  "keywords": ["typescript", "library", "tool"],
  "author": {
    "name": "John Doe",
    "email": "john@example.com",
    "url": "https://example.com"
  },
  "license": "MIT",
  "repository": {
    "type": "git",
    "url": "https://github.com/org/repo.git"
  },
  "bugs": {
    "url": "https://github.com/org/repo/issues",
    "email": "bugs@example.com"
  },
  "dependencies": {
    "zod": "^3.22.0",
    "typescript": "^5.0.0",
    "axios": "^1.6.0",
    "lodash": "^4.17.21"
  },
  "devDependencies": {
    "vitest": "^1.0.0",
    "prettier": "^3.0.0",
    "jest": "^29.0.0",
    "eslint": "^8.0.0",
    "@types/node": "^20.0.0"
  },
  "engines": {
    "npm": ">=8.0.0",
    "node": ">=18.0.0"
  }
}"#;

const LARGE_PACKAGE_JSON: &str = r#"{
  "version": "3.10.5",
  "description": "A comprehensive package.json with many fields",
  "displayName": "Large Test Package",
  "main": "dist/cjs/index.js",
  "module": "dist/esm/index.js",
  "types": "dist/types/index.d.ts",
  "exports": {
    ".": {
      "types": "./dist/types/index.d.ts",
      "import": "./dist/esm/index.js",
      "require": "./dist/cjs/index.js",
      "default": "./dist/cjs/index.js"
    },
    "./utils": {
      "types": "./dist/types/utils.d.ts",
      "import": "./dist/esm/utils.js",
      "require": "./dist/cjs/utils.js"
    }
  },
  "scripts": {
    "test": "jest --coverage",
    "test:watch": "jest --watch",
    "test:ci": "jest --ci --coverage",
    "build": "npm run build:cjs && npm run build:esm && npm run build:types",
    "build:cjs": "tsc -p tsconfig.cjs.json",
    "build:esm": "tsc -p tsconfig.esm.json",
    "build:types": "tsc -p tsconfig.types.json",
    "lint": "eslint src/**/*.ts",
    "lint:fix": "eslint src/**/*.ts --fix",
    "format": "prettier --write .",
    "format:check": "prettier --check .",
    "typecheck": "tsc --noEmit",
    "prepublishOnly": "npm run build && npm test",
    "pretest": "npm run lint && npm run typecheck",
    "clean": "rimraf dist coverage",
    "docs": "typedoc --out docs src",
    "release": "semantic-release"
  },
  "name": "@scope/large-package",
  "private": false,
  "keywords": [
    "typescript",
    "library",
    "utility",
    "tool",
    "performance",
    "benchmark",
    "test"
  ],
  "homepage": "https://github.com/org/large-package#readme",
  "author": {
    "name": "Jane Smith",
    "email": "jane@example.com",
    "url": "https://janesmith.dev"
  },
  "maintainers": [
    {
      "name": "Jane Smith",
      "email": "jane@example.com"
    },
    {
      "name": "Bob Johnson",
      "email": "bob@example.com"
    }
  ],
  "contributors": [
    {
      "name": "Alice Cooper",
      "email": "alice@example.com"
    }
  ],
  "license": "MIT",
  "repository": {
    "type": "git",
    "url": "https://github.com/org/large-package.git"
  },
  "bugs": {
    "url": "https://github.com/org/large-package/issues",
    "email": "bugs@example.com"
  },
  "funding": {
    "type": "opencollective",
    "url": "https://opencollective.com/large-package"
  },
  "files": [
    "dist",
    "src",
    "README.md",
    "LICENSE"
  ],
  "dependencies": {
    "zod": "^3.22.0",
    "typescript": "^5.0.0",
    "axios": "^1.6.0",
    "lodash": "^4.17.21",
    "date-fns": "^2.30.0",
    "react": "^18.0.0"
  },
  "devDependencies": {
    "vitest": "^1.0.0",
    "prettier": "^3.0.0",
    "jest": "^29.0.0",
    "eslint": "^8.0.0",
    "@types/node": "^20.0.0",
    "@types/react": "^18.0.0",
    "typescript": "^5.0.0",
    "rimraf": "^5.0.0",
    "semantic-release": "^22.0.0",
    "typedoc": "^0.25.0"
  },
  "peerDependencies": {
    "react": "^18.0.0"
  },
  "peerDependenciesMeta": {
    "react": {
      "optional": true
    }
  },
  "engines": {
    "npm": ">=8.0.0",
    "node": ">=18.0.0"
  },
  "browserslist": [
    ">0.2%",
    "not dead",
    "not op_mini all"
  ],
  "prettier": {
    "semi": true,
    "singleQuote": true,
    "tabWidth": 2,
    "trailingComma": "es5"
  },
  "eslintConfig": {
    "extends": ["eslint:recommended", "plugin:@typescript-eslint/recommended"],
    "parser": "@typescript-eslint/parser",
    "plugins": ["@typescript-eslint"]
  },
  "jest": {
    "preset": "ts-jest",
    "testEnvironment": "node",
    "collectCoverageFrom": ["src/**/*.ts"]
  }
}"#;

fn bench_small_package(c: &mut Criterion) {
    c.bench_function("sort_small_package", |b| {
        b.iter(|| sort_package_json(black_box(SMALL_PACKAGE_JSON)))
    });
}

fn bench_medium_package(c: &mut Criterion) {
    c.bench_function("sort_medium_package", |b| {
        b.iter(|| sort_package_json(black_box(MEDIUM_PACKAGE_JSON)))
    });
}

fn bench_large_package(c: &mut Criterion) {
    c.bench_function("sort_large_package", |b| {
        b.iter(|| sort_package_json(black_box(LARGE_PACKAGE_JSON)))
    });
}

fn bench_idempotency(c: &mut Criterion) {
    let sorted = sort_package_json(MEDIUM_PACKAGE_JSON).unwrap();
    c.bench_function("sort_already_sorted", |b| {
        b.iter(|| sort_package_json(black_box(&sorted)))
    });
}

fn bench_parametric(c: &mut Criterion) {
    let mut group = c.benchmark_group("sort_by_size");
    
    for (name, input) in [
        ("small", SMALL_PACKAGE_JSON),
        ("medium", MEDIUM_PACKAGE_JSON),
        ("large", LARGE_PACKAGE_JSON),
    ] {
        group.bench_with_input(BenchmarkId::from_parameter(name), &input, |b, &input| {
            b.iter(|| sort_package_json(black_box(input)));
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_small_package,
    bench_medium_package,
    bench_large_package,
    bench_idempotency,
    bench_parametric
);
criterion_main!(benches);
