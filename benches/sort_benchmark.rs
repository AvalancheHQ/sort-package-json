use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use sort_package_json::sort_package_json;

const SMALL_PACKAGE_JSON: &str = r#"{
  "version": "1.0.0",
  "name": "small-package",
  "description": "A small test package",
  "main": "index.js",
  "scripts": {
    "test": "jest",
    "build": "webpack"
  },
  "dependencies": {
    "react": "^18.0.0",
    "lodash": "^4.17.21"
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
  "maintainers": [
    {
      "url": "https://maintainer1.com",
      "name": "Maintainer One",
      "email": "maintainer1@example.com"
    }
  ],
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
  "name": "large-package",
  "version": "2.5.3",
  "description": "A comprehensive package with many fields",
  "keywords": ["test", "benchmark", "sorting", "json", "large", "comprehensive"],
  "homepage": "https://github.com/test/large-package#readme",
  "bugs": {
    "email": "bugs@example.com",
    "url": "https://github.com/test/large-package/issues"
  },
  "repository": {
    "type": "git",
    "url": "https://github.com/test/large-package"
  },
  "funding": {
    "type": "opencollective",
    "url": "https://opencollective.com/large-package"
  },
  "license": "MIT",
  "author": {
    "name": "Test Author",
    "email": "author@example.com",
    "url": "https://example.com"
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
    "./utils": {
      "types": "./dist/utils.d.ts",
      "import": "./dist/utils.esm.js",
      "default": "./dist/utils.js"
    },
    "./package.json": "./package.json"
  },
  "main": "./dist/index.js",
  "module": "./dist/index.esm.js",
  "types": "./dist/index.d.ts",
  "browser": "./dist/browser.js",
  "bin": {
    "large-cli": "./bin/cli.js"
  },
  "directories": {
    "lib": "./lib",
    "bin": "./bin",
    "doc": "./docs",
    "test": "./tests"
  },
  "files": ["src", "dist", "bin", "README.md", "LICENSE"],
  "workspaces": ["packages/*"],
  "scripts": {
    "prebuild": "npm run clean",
    "build": "webpack",
    "postbuild": "npm run types",
    "clean": "rimraf dist",
    "dev": "webpack serve --mode development",
    "lint": "eslint .",
    "lint:fix": "eslint . --fix",
    "pretest": "npm run lint",
    "test": "jest",
    "test:coverage": "jest --coverage",
    "posttest": "npm run test:coverage",
    "types": "tsc --emitDeclarationOnly",
    "prepare": "husky install"
  },
  "husky": {
    "hooks": {
      "pre-commit": "lint-staged"
    }
  },
  "lint-staged": {
    "*.js": ["eslint --fix", "prettier --write"]
  },
  "prettier": {
    "semi": true,
    "singleQuote": true,
    "trailingComma": "es5"
  },
  "eslintConfig": {
    "extends": ["eslint:recommended"],
    "env": {
      "node": true,
      "es6": true
    }
  },
  "jest": {
    "testEnvironment": "node",
    "coverageDirectory": "./coverage",
    "collectCoverageFrom": ["src/**/*.js"]
  },
  "dependencies": {
    "axios": "^1.0.0",
    "lodash": "^4.17.21",
    "react": "^18.0.0",
    "react-dom": "^18.0.0"
  },
  "devDependencies": {
    "eslint": "^8.0.0",
    "husky": "^8.0.0",
    "jest": "^29.0.0",
    "lint-staged": "^13.0.0",
    "prettier": "^2.8.0",
    "rimraf": "^4.0.0",
    "typescript": "^5.0.0",
    "webpack": "^5.0.0",
    "webpack-cli": "^5.0.0"
  },
  "peerDependencies": {
    "react": ">=17.0.0"
  },
  "engines": {
    "node": ">=18.0.0",
    "npm": ">=8.0.0"
  },
  "publishConfig": {
    "access": "public",
    "registry": "https://registry.npmjs.org/"
  },
  "customField": "this is a custom unknown field",
  "anotherCustom": "more custom data",
  "_internal": "private field one",
  "_custom": "another private field",
  "_id": "large-package@2.5.3"
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

fn bench_sort_multiple_iterations(c: &mut Criterion) {
    let mut group = c.benchmark_group("sort_iterations");
    
    for size in [("small", SMALL_PACKAGE_JSON), ("medium", MEDIUM_PACKAGE_JSON), ("large", LARGE_PACKAGE_JSON)].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size.0), &size.1, |b, &input| {
            b.iter(|| {
                // Simulate sorting the same file multiple times
                let _result1 = sort_package_json(black_box(input));
                let _result2 = sort_package_json(black_box(input));
            });
        });
    }
    
    group.finish();
}

fn bench_sort_idempotency(c: &mut Criterion) {
    c.bench_function("sort_idempotency", |b| {
        // First sort the input
        let sorted_once = sort_package_json(MEDIUM_PACKAGE_JSON).unwrap();
        
        // Benchmark sorting the already-sorted input
        b.iter(|| sort_package_json(black_box(&sorted_once)))
    });
}

criterion_group!(
    benches,
    bench_sort_small,
    bench_sort_medium,
    bench_sort_large,
    bench_sort_multiple_iterations,
    bench_sort_idempotency
);
criterion_main!(benches);
