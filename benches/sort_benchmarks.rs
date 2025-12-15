use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

const SMALL_PACKAGE_JSON: &str = r#"{
  "name": "small-package",
  "version": "1.0.0",
  "description": "A small test package",
  "main": "index.js",
  "dependencies": {
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
  "exports": {
    ".": {
      "default": "./dist/index.js",
      "require": "./dist/index.cjs",
      "types": "./dist/index.d.ts",
      "import": "./dist/index.esm.js"
    }
  },
  "main": "./dist/index.js",
  "scripts": {
    "test": "jest",
    "build": "webpack"
  },
  "dependencies": {
    "react": "^18.0.0",
    "axios": "^1.0.0"
  },
  "devDependencies": {
    "webpack": "^5.0.0",
    "jest": "^29.0.0"
  }
}"#;

const LARGE_PACKAGE_JSON: &str = r#"{
  "name": "large-package",
  "version": "2.5.3",
  "description": "A comprehensive package with many fields",
  "keywords": ["large", "comprehensive", "test", "benchmark", "performance"],
  "homepage": "https://github.com/org/large-package#readme",
  "bugs": {
    "email": "support@example.com",
    "url": "https://github.com/org/large-package/issues"
  },
  "repository": {
    "url": "https://github.com/org/large-package",
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
    "name": "Large Package Author"
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
    "dev": "webpack serve",
    "format": "prettier --write .",
    "typecheck": "tsc --noEmit"
  },
  "husky": {
    "hooks": {
      "pre-commit": "lint-staged"
    }
  },
  "lint-staged": {
    "*.{js,jsx,ts,tsx}": ["eslint --fix", "prettier --write"]
  },
  "dependencies": {
    "react": "^18.2.0",
    "axios": "^1.4.0",
    "lodash": "^4.17.21",
    "express": "^4.18.2",
    "dotenv": "^16.0.3"
  },
  "devDependencies": {
    "webpack": "^5.88.0",
    "jest": "^29.6.0",
    "typescript": "^5.1.6",
    "eslint": "^8.45.0",
    "prettier": "^3.0.0"
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
    "plugins": ["@babel/plugin-proposal-class-properties"]
  },
  "jest": {
    "testEnvironment": "jsdom",
    "collectCoverage": true
  },
  "publishConfig": {
    "access": "public"
  },
  "customField": "this is a custom unknown field",
  "anotherCustom": "another custom field",
  "_custom": "another private field",
  "_id": "test-package@1.0.0"
}"#;

fn bench_small_package(c: &mut Criterion) {
    c.bench_function("sort_small_package", |b| {
        b.iter(|| sort_package_json(SMALL_PACKAGE_JSON));
    });
}

fn bench_medium_package(c: &mut Criterion) {
    c.bench_function("sort_medium_package", |b| {
        b.iter(|| sort_package_json(MEDIUM_PACKAGE_JSON));
    });
}

fn bench_large_package(c: &mut Criterion) {
    c.bench_function("sort_large_package", |b| {
        b.iter(|| sort_package_json(LARGE_PACKAGE_JSON));
    });
}

fn bench_already_sorted(c: &mut Criterion) {
    // Sort once to get the sorted version
    let sorted = sort_package_json(MEDIUM_PACKAGE_JSON).unwrap();
    c.bench_function("sort_already_sorted", |b| {
        b.iter(|| sort_package_json(&sorted));
    });
}

fn bench_real_world_fixture(c: &mut Criterion) {
    let fixture = include_str!("../tests/fixtures/package.json");
    c.bench_function("sort_real_world_fixture", |b| {
        b.iter(|| sort_package_json(fixture));
    });
}

criterion_group!(
    benches,
    bench_small_package,
    bench_medium_package,
    bench_large_package,
    bench_already_sorted,
    bench_real_world_fixture
);
criterion_main!(benches);
