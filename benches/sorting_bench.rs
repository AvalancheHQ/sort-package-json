use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

// Small package.json with basic fields
const SMALL_PACKAGE: &str = r#"{
  "version": "1.0.0",
  "name": "small-package",
  "dependencies": {
    "react": "^18.0.0",
    "lodash": "^4.17.21"
  },
  "scripts": {
    "test": "jest",
    "build": "webpack"
  }
}"#;

// Medium package.json with more fields and complexity
const MEDIUM_PACKAGE: &str = r#"{
  "name": "medium-package",
  "version": "2.1.0",
  "description": "A medium-sized package for benchmarking",
  "keywords": ["test", "benchmark", "sorting", "json", "performance"],
  "homepage": "https://github.com/test/medium-package#readme",
  "bugs": {
    "email": "bugs@example.com",
    "url": "https://github.com/test/medium-package/issues"
  },
  "repository": {
    "url": "https://github.com/test/medium-package",
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
      "types": "./dist/utils.d.ts"
    }
  },
  "main": "./dist/index.js",
  "module": "./dist/index.esm.js",
  "types": "./dist/index.d.ts",
  "scripts": {
    "test": "jest",
    "build": "webpack",
    "lint": "eslint .",
    "dev": "webpack serve"
  },
  "dependencies": {
    "react": "^18.0.0",
    "axios": "^1.0.0",
    "lodash": "^4.17.21"
  },
  "devDependencies": {
    "webpack": "^5.0.0",
    "jest": "^29.0.0",
    "typescript": "^5.0.0"
  },
  "engines": {
    "node": ">=18.0.0"
  }
}"#;

// Large package.json with many dependencies and complex structure
const LARGE_PACKAGE: &str = r#"{
  "$schema": "https://json.schemastore.org/package",
  "name": "large-package",
  "displayName": "Large Package for Benchmarking",
  "version": "3.5.2",
  "stableVersion": "3.5.0",
  "private": false,
  "description": "A large package with many fields for comprehensive benchmarking",
  "categories": ["utilities", "performance", "testing"],
  "keywords": ["test", "benchmark", "sorting", "json", "performance", "optimization", "utilities"],
  "homepage": "https://github.com/test/large-package#readme",
  "bugs": {
    "email": "bugs@example.com",
    "url": "https://github.com/test/large-package/issues"
  },
  "repository": {
    "url": "https://github.com/test/large-package.git",
    "type": "git"
  },
  "funding": {
    "type": "opencollective",
    "url": "https://opencollective.com/large-package"
  },
  "license": "MIT",
  "author": {
    "url": "https://example.com",
    "email": "author@example.com",
    "name": "Test Author"
  },
  "maintainers": [
    {
      "name": "Maintainer One",
      "email": "maintainer1@example.com",
      "url": "https://maintainer1.com"
    },
    {
      "name": "Maintainer Two",
      "email": "maintainer2@example.com",
      "url": "https://maintainer2.com"
    }
  ],
  "contributors": [
    {
      "name": "Contributor One",
      "email": "contributor1@example.com"
    }
  ],
  "sideEffects": false,
  "type": "module",
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
    },
    "./internal": {
      "types": "./dist/internal.d.ts",
      "import": "./dist/internal.esm.js"
    }
  },
  "main": "./dist/index.js",
  "module": "./dist/index.esm.js",
  "unpkg": "./dist/index.umd.js",
  "types": "./dist/index.d.ts",
  "typings": "./dist/index.d.ts",
  "files": [
    "dist",
    "src",
    "README.md",
    "LICENSE",
    "CHANGELOG.md"
  ],
  "workspaces": [
    "packages/*"
  ],
  "scripts": {
    "build": "webpack",
    "build:prod": "webpack --mode production",
    "clean": "rm -rf dist",
    "dev": "webpack serve",
    "lint": "eslint .",
    "lint:fix": "eslint . --fix",
    "prebuild": "npm run clean",
    "postbuild": "npm run test",
    "pretest": "npm run lint",
    "test": "jest",
    "test:watch": "jest --watch",
    "test:coverage": "jest --coverage",
    "posttest": "echo 'Tests complete'",
    "typecheck": "tsc --noEmit"
  },
  "husky": {
    "hooks": {
      "pre-commit": "lint-staged"
    }
  },
  "lint-staged": {
    "*.{js,ts}": ["eslint --fix", "prettier --write"]
  },
  "prettier": {
    "semi": true,
    "singleQuote": true,
    "tabWidth": 2
  },
  "eslintConfig": {
    "extends": ["eslint:recommended", "plugin:@typescript-eslint/recommended"],
    "parser": "@typescript-eslint/parser",
    "plugins": ["@typescript-eslint"]
  },
  "jest": {
    "preset": "ts-jest",
    "testEnvironment": "node",
    "coverageDirectory": "coverage"
  },
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "axios": "^1.4.0",
    "lodash": "^4.17.21",
    "date-fns": "^2.30.0",
    "classnames": "^2.3.2"
  },
  "devDependencies": {
    "@types/node": "^20.0.0",
    "@types/react": "^18.2.0",
    "@typescript-eslint/eslint-plugin": "^5.60.0",
    "@typescript-eslint/parser": "^5.60.0",
    "eslint": "^8.44.0",
    "husky": "^8.0.3",
    "jest": "^29.5.0",
    "lint-staged": "^13.2.3",
    "prettier": "^3.0.0",
    "ts-jest": "^29.1.1",
    "typescript": "^5.1.6",
    "webpack": "^5.88.0",
    "webpack-cli": "^5.1.4",
    "webpack-dev-server": "^4.15.1"
  },
  "peerDependencies": {
    "react": ">=17.0.0"
  },
  "engines": {
    "node": ">=18.0.0",
    "npm": ">=8.0.0"
  },
  "packageManager": "npm@9.8.1",
  "publishConfig": {
    "access": "public",
    "registry": "https://registry.npmjs.org/"
  },
  "customField": "custom value",
  "anotherCustom": "another custom",
  "_internal": "private field",
  "_id": "large-package@3.5.2"
}"#;

fn bench_small_package(c: &mut Criterion) {
    c.bench_function("sort_small_package", |b| {
        b.iter(|| sort_package_json(black_box(SMALL_PACKAGE)))
    });
}

fn bench_medium_package(c: &mut Criterion) {
    c.bench_function("sort_medium_package", |b| {
        b.iter(|| sort_package_json(black_box(MEDIUM_PACKAGE)))
    });
}

fn bench_large_package(c: &mut Criterion) {
    c.bench_function("sort_large_package", |b| {
        b.iter(|| sort_package_json(black_box(LARGE_PACKAGE)))
    });
}

fn bench_idempotency(c: &mut Criterion) {
    let sorted = sort_package_json(MEDIUM_PACKAGE).unwrap();
    c.bench_function("sort_already_sorted", |b| {
        b.iter(|| sort_package_json(black_box(&sorted)))
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
