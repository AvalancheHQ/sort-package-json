use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

const SMALL_PACKAGE_JSON: &str = r#"{
  "name": "small-package",
  "version": "1.0.0",
  "description": "A small test package",
  "main": "index.js",
  "scripts": {
    "test": "jest",
    "build": "webpack"
  },
  "dependencies": {
    "react": "^18.0.0"
  }
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
  "exports": {
    ".": {
      "default": "./dist/index.js",
      "require": "./dist/index.cjs",
      "types": "./dist/index.d.ts",
      "import": "./dist/index.esm.js"
    }
  },
  "main": "./dist/index.js",
  "module": "./dist/index.esm.js",
  "types": "./dist/index.d.ts",
  "files": ["src", "dist", "README.md"],
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
  }
}"#;

const LARGE_PACKAGE_JSON: &str = r#"{
  "name": "large-package",
  "version": "2.5.1",
  "description": "A comprehensive package.json for testing",
  "keywords": ["test", "sorting", "json", "benchmark", "performance", "npm", "package"],
  "homepage": "https://github.com/test/large-package#readme",
  "bugs": {
    "email": "bugs@example.com",
    "url": "https://github.com/test/large-package/issues"
  },
  "repository": {
    "url": "https://github.com/test/large-package",
    "type": "git",
    "directory": "packages/core"
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
      "email": "contrib1@example.com"
    }
  ],
  "type": "module",
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
  "browser": "./dist/browser.js",
  "types": "./dist/index.d.ts",
  "bin": {
    "my-cli": "./bin/cli.js"
  },
  "files": ["src", "dist", "README.md", "LICENSE", "bin"],
  "workspaces": ["packages/*"],
  "scripts": {
    "test": "jest",
    "test:watch": "jest --watch",
    "test:coverage": "jest --coverage",
    "build": "webpack",
    "build:prod": "webpack --mode production",
    "lint": "eslint .",
    "lint:fix": "eslint . --fix",
    "format": "prettier --write .",
    "typecheck": "tsc --noEmit",
    "dev": "webpack serve",
    "prepublishOnly": "npm run build",
    "postinstall": "node scripts/postinstall.js"
  },
  "husky": {
    "hooks": {
      "pre-commit": "lint-staged"
    }
  },
  "lint-staged": {
    "*.{js,jsx,ts,tsx}": ["eslint --fix", "prettier --write"],
    "*.{json,md}": ["prettier --write"]
  },
  "prettier": {
    "semi": true,
    "singleQuote": true,
    "trailingComma": "es5"
  },
  "eslintConfig": {
    "extends": ["eslint:recommended", "plugin:react/recommended"],
    "env": {
      "browser": true,
      "node": true,
      "es2021": true
    }
  },
  "jest": {
    "testEnvironment": "node",
    "coverageDirectory": "coverage",
    "collectCoverageFrom": ["src/**/*.{js,jsx,ts,tsx}"]
  },
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "axios": "^1.4.0",
    "lodash": "^4.17.21",
    "express": "^4.18.2",
    "uuid": "^9.0.0"
  },
  "devDependencies": {
    "webpack": "^5.88.0",
    "webpack-cli": "^5.1.4",
    "webpack-dev-server": "^4.15.1",
    "jest": "^29.6.0",
    "typescript": "^5.1.6",
    "eslint": "^8.45.0",
    "prettier": "^3.0.0",
    "@types/node": "^20.4.0",
    "@types/react": "^18.2.14",
    "@babel/core": "^7.22.9",
    "husky": "^8.0.3",
    "lint-staged": "^13.2.3"
  },
  "peerDependencies": {
    "react": ">=16.8.0"
  },
  "optionalDependencies": {
    "fsevents": "^2.3.2"
  },
  "engines": {
    "npm": ">=8.0.0",
    "node": ">=18.0.0"
  },
  "packageManager": "npm@9.8.1",
  "os": ["darwin", "linux", "win32"],
  "publishConfig": {
    "access": "public",
    "registry": "https://registry.npmjs.org/"
  },
  "customField": "this is a custom unknown field",
  "anotherCustom": "alphabetically before customField",
  "_custom": "another private field",
  "_id": "test-package@1.0.0"
}"#;

fn bench_sort_small_package(c: &mut Criterion) {
    c.bench_function("sort_small_package", |b| {
        b.iter(|| sort_package_json(black_box(SMALL_PACKAGE_JSON)))
    });
}

fn bench_sort_medium_package(c: &mut Criterion) {
    c.bench_function("sort_medium_package", |b| {
        b.iter(|| sort_package_json(black_box(MEDIUM_PACKAGE_JSON)))
    });
}

fn bench_sort_large_package(c: &mut Criterion) {
    c.bench_function("sort_large_package", |b| {
        b.iter(|| sort_package_json(black_box(LARGE_PACKAGE_JSON)))
    });
}

fn bench_sort_idempotency(c: &mut Criterion) {
    c.bench_function("sort_idempotency", |b| {
        let sorted = sort_package_json(MEDIUM_PACKAGE_JSON).unwrap();
        b.iter(|| sort_package_json(black_box(&sorted)))
    });
}

criterion_group!(
    benches,
    bench_sort_small_package,
    bench_sort_medium_package,
    bench_sort_large_package,
    bench_sort_idempotency
);
criterion_main!(benches);
