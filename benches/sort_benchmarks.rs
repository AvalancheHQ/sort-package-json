use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

const SMALL_PACKAGE_JSON: &str = r#"{
  "version": "1.0.0",
  "dependencies": { "lodash": "^4.17.21" },
  "name": "my-package",
  "scripts": { "test": "jest", "build": "webpack" }
}"#;

const MEDIUM_PACKAGE_JSON: &str = r#"{
  "name": "test-package",
  "version": "1.0.0",
  "description": "A test package for sorting",
  "keywords": ["test", "sorting", "json", "test", "example"],
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
    "lodash": "^4.17.21"
  },
  "devDependencies": {
    "webpack": "^5.0.0",
    "jest": "^29.0.0",
    "typescript": "^5.0.0"
  },
  "engines": {
    "npm": ">=8.0.0",
    "node": ">=18.0.0"
  }
}"#;

const LARGE_PACKAGE_JSON: &str = r#"{
  "name": "large-monorepo-package",
  "version": "2.5.1",
  "description": "A comprehensive package with many fields",
  "keywords": ["test", "sorting", "json", "monorepo", "example", "benchmark", "performance"],
  "homepage": "https://github.com/test/large-package#readme",
  "bugs": {
    "email": "bugs@example.com",
    "url": "https://github.com/test/large-package/issues"
  },
  "repository": {
    "url": "https://github.com/test/large-package",
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
  "contributors": [
    {
      "name": "Contributor One",
      "email": "contributor1@example.com"
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
  "bin": {
    "cli-tool": "./bin/cli.js"
  },
  "directories": {
    "lib": "lib",
    "bin": "bin",
    "man": "man",
    "doc": "docs",
    "example": "examples",
    "test": "test"
  },
  "files": ["src", "dist", "README.md", "LICENSE", "bin", "lib"],
  "workspaces": ["packages/*"],
  "scripts": {
    "test": "jest",
    "test:unit": "jest --testPathPattern=unit",
    "test:integration": "jest --testPathPattern=integration",
    "test:e2e": "playwright test",
    "posttest": "echo 'Tests complete'",
    "build": "webpack",
    "build:prod": "webpack --mode production",
    "build:dev": "webpack --mode development",
    "lint": "eslint .",
    "lint:fix": "eslint . --fix",
    "format": "prettier --write .",
    "format:check": "prettier --check .",
    "pretest": "echo 'Starting tests'",
    "dev": "webpack serve",
    "start": "node dist/index.js",
    "clean": "rm -rf dist",
    "prepare": "husky install",
    "prepublishOnly": "npm run build"
  },
  "husky": {
    "hooks": {
      "pre-commit": "lint-staged"
    }
  },
  "lint-staged": {
    "*.{js,jsx,ts,tsx}": ["eslint --fix", "prettier --write"],
    "*.{json,md,yml,yaml}": ["prettier --write"]
  },
  "commitlint": {
    "extends": ["@commitlint/config-conventional"]
  },
  "prettier": {
    "semi": true,
    "singleQuote": true,
    "trailingComma": "es5",
    "printWidth": 100
  },
  "eslintConfig": {
    "extends": ["eslint:recommended", "plugin:@typescript-eslint/recommended"],
    "parser": "@typescript-eslint/parser",
    "plugins": ["@typescript-eslint"],
    "rules": {
      "no-console": "warn"
    }
  },
  "jest": {
    "preset": "ts-jest",
    "testEnvironment": "node",
    "collectCoverage": true,
    "coverageDirectory": "coverage",
    "testMatch": ["**/__tests__/**/*.ts", "**/?(*.)+(spec|test).ts"]
  },
  "dependencies": {
    "react": "^18.0.0",
    "react-dom": "^18.0.0",
    "axios": "^1.0.0",
    "lodash": "^4.17.21",
    "express": "^4.18.0",
    "mongoose": "^7.0.0",
    "dotenv": "^16.0.0"
  },
  "devDependencies": {
    "webpack": "^5.0.0",
    "webpack-cli": "^5.0.0",
    "webpack-dev-server": "^4.0.0",
    "jest": "^29.0.0",
    "ts-jest": "^29.0.0",
    "typescript": "^5.0.0",
    "@typescript-eslint/eslint-plugin": "^6.0.0",
    "@typescript-eslint/parser": "^6.0.0",
    "eslint": "^8.0.0",
    "prettier": "^3.0.0",
    "husky": "^8.0.0",
    "lint-staged": "^14.0.0",
    "@commitlint/cli": "^17.0.0",
    "@commitlint/config-conventional": "^17.0.0"
  },
  "peerDependencies": {
    "react": ">=18.0.0"
  },
  "optionalDependencies": {
    "fsevents": "^2.3.2"
  },
  "engines": {
    "node": ">=18.0.0",
    "npm": ">=8.0.0"
  },
  "publishConfig": {
    "access": "public",
    "registry": "https://registry.npmjs.org/"
  },
  "babel": {
    "presets": ["@babel/preset-env", "@babel/preset-react", "@babel/preset-typescript"],
    "plugins": ["@babel/plugin-proposal-class-properties"]
  },
  "browserslist": [
    "> 1%",
    "last 2 versions",
    "not dead"
  ],
  "customField1": "custom value 1",
  "customField2": "custom value 2",
  "customField3": "custom value 3",
  "_custom": "private field",
  "_id": "large-monorepo-package@2.5.1",
  "_internal": "internal data"
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

fn bench_idempotent_sort(c: &mut Criterion) {
    // First, sort once to get a sorted version
    let sorted_once = sort_package_json(MEDIUM_PACKAGE_JSON).unwrap();

    c.bench_function("sort_already_sorted", |b| {
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
