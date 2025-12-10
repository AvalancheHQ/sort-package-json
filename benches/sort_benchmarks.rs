use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

const SMALL_PACKAGE_JSON: &str = r#"{
  "version": "1.0.0",
  "dependencies": {
    "react": "^18.0.0",
    "axios": "^1.0.0"
  },
  "name": "simple-package",
  "scripts": {
    "build": "webpack",
    "test": "jest"
  }
}"#;

const MEDIUM_PACKAGE_JSON: &str = r#"{
  "name": "test-package",
  "version": "1.0.0",
  "description": "A test package for sorting",
  "keywords": [
    "test",
    "sorting",
    "json",
    "test",
    "example"
  ],
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
    },
    "./package.json": "./package.json"
  },
  "main": "./dist/index.js",
  "types": "./dist/index.d.ts",
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
  }
}"#;

const LARGE_PACKAGE_JSON: &str = r#"{
  "name": "large-test-package",
  "version": "2.1.0",
  "description": "A comprehensive test package with many fields",
  "keywords": [
    "test",
    "sorting",
    "json",
    "benchmark",
    "example",
    "comprehensive",
    "test"
  ],
  "homepage": "https://github.com/test/test-package#readme",
  "bugs": {
    "email": "bugs@example.com",
    "url": "https://github.com/test/test-package/issues"
  },
  "repository": {
    "url": "https://github.com/test/test-package",
    "type": "git"
  },
  "funding": {
    "url": "https://github.com/sponsors/test",
    "type": "github"
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
    },
    "./components": {
      "import": "./dist/components.esm.js",
      "default": "./dist/components.js",
      "types": "./dist/components.d.ts"
    }
  },
  "main": "./dist/index.js",
  "module": "./dist/index.esm.js",
  "types": "./dist/index.d.ts",
  "browser": "./dist/index.browser.js",
  "files": [
    "src",
    "dist",
    "README.md",
    "dist",
    "LICENSE",
    "CHANGELOG.md"
  ],
  "scripts": {
    "test": "jest",
    "test:watch": "jest --watch",
    "test:coverage": "jest --coverage",
    "posttest": "echo 'Tests complete'",
    "build": "webpack",
    "build:prod": "NODE_ENV=production webpack",
    "lint": "eslint .",
    "lint:fix": "eslint . --fix",
    "format": "prettier --write .",
    "pretest": "echo 'Starting tests'",
    "prebuild": "npm run clean",
    "postbuild": "npm run validate",
    "clean": "rm -rf dist",
    "validate": "npm run lint && npm run test",
    "dev": "webpack serve",
    "prepare": "husky install"
  },
  "dependencies": {
    "react": "^18.0.0",
    "react-dom": "^18.0.0",
    "axios": "^1.0.0",
    "lodash": "^4.17.21",
    "classnames": "^2.3.2",
    "prop-types": "^15.8.1"
  },
  "devDependencies": {
    "webpack": "^5.0.0",
    "webpack-cli": "^5.0.0",
    "webpack-dev-server": "^4.0.0",
    "jest": "^29.0.0",
    "typescript": "^5.0.0",
    "eslint": "^8.0.0",
    "prettier": "^3.0.0",
    "husky": "^8.0.0",
    "@babel/core": "^7.22.0",
    "@babel/preset-env": "^7.22.0",
    "@babel/preset-react": "^7.22.0"
  },
  "peerDependencies": {
    "react": ">=16.8.0"
  },
  "engines": {
    "npm": ">=8.0.0",
    "node": ">=18.0.0"
  },
  "babel": {
    "presets": ["@babel/preset-env", "@babel/preset-react"],
    "plugins": [
      "@babel/plugin-proposal-class-properties",
      "@babel/plugin-transform-runtime"
    ]
  },
  "jest": {
    "testEnvironment": "jsdom",
    "collectCoverageFrom": ["src/**/*.{js,jsx}"],
    "coverageThreshold": {
      "global": {
        "branches": 80,
        "functions": 80,
        "lines": 80,
        "statements": 80
      }
    }
  },
  "eslintConfig": {
    "extends": ["eslint:recommended", "plugin:react/recommended"],
    "parserOptions": {
      "ecmaVersion": 2021,
      "sourceType": "module"
    }
  },
  "prettier": {
    "semi": true,
    "singleQuote": true,
    "tabWidth": 2,
    "trailingComma": "es5"
  },
  "publishConfig": {
    "registry": "https://registry.npmjs.org/",
    "access": "public"
  },
  "customField": "this is a custom unknown field",
  "anotherCustom": "another custom field",
  "_custom": "another private field",
  "_id": "test-package@1.0.0",
  "_internal": "internal data"
}"#;

fn bench_sort_small_package(c: &mut Criterion) {
    c.bench_function("sort_small_package", |b| {
        b.iter(|| {
            sort_package_json(black_box(SMALL_PACKAGE_JSON))
                .expect("Failed to sort small package.json")
        });
    });
}

fn bench_sort_medium_package(c: &mut Criterion) {
    c.bench_function("sort_medium_package", |b| {
        b.iter(|| {
            sort_package_json(black_box(MEDIUM_PACKAGE_JSON))
                .expect("Failed to sort medium package.json")
        });
    });
}

fn bench_sort_large_package(c: &mut Criterion) {
    c.bench_function("sort_large_package", |b| {
        b.iter(|| {
            sort_package_json(black_box(LARGE_PACKAGE_JSON))
                .expect("Failed to sort large package.json")
        });
    });
}

fn bench_sort_already_sorted(c: &mut Criterion) {
    // First sort it, then benchmark sorting it again
    let already_sorted =
        sort_package_json(MEDIUM_PACKAGE_JSON).expect("Failed to pre-sort package.json");
    c.bench_function("sort_already_sorted", |b| {
        b.iter(|| {
            sort_package_json(black_box(&already_sorted))
                .expect("Failed to sort already sorted package.json")
        });
    });
}

criterion_group!(
    benches,
    bench_sort_small_package,
    bench_sort_medium_package,
    bench_sort_large_package,
    bench_sort_already_sorted
);
criterion_main!(benches);
