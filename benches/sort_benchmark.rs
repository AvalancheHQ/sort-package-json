use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

const SIMPLE_PACKAGE_JSON: &str = r#"{
  "name": "simple-package",
  "version": "1.0.0",
  "dependencies": {
    "react": "^18.0.0",
    "axios": "^1.0.0"
  }
}"#;

const COMPLEX_PACKAGE_JSON: &str = r#"{
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
  "files": [
    "src",
    "dist",
    "README.md",
    "dist",
    "LICENSE"
  ],
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
  },
  "babel": {
    "presets": ["@babel/preset-env", "@babel/preset-react"],
    "plugins": ["@babel/plugin-proposal-class-properties"]
  },
  "publishConfig": {
    "access": "public"
  },
  "customField": "this is a custom unknown field",
  "_custom": "another private field",
  "_id": "test-package@1.0.0"
}"#;

fn bench_simple_package_json(c: &mut Criterion) {
    c.bench_function("sort_simple_package_json", |b| {
        b.iter(|| sort_package_json(SIMPLE_PACKAGE_JSON))
    });
}

fn bench_complex_package_json(c: &mut Criterion) {
    c.bench_function("sort_complex_package_json", |b| {
        b.iter(|| sort_package_json(COMPLEX_PACKAGE_JSON))
    });
}

fn bench_nested_exports(c: &mut Criterion) {
    let nested_exports = r#"{
  "name": "exports-package",
  "exports": {
    ".": {
      "default": "./dist/index.js",
      "require": "./dist/index.cjs",
      "types": "./dist/index.d.ts",
      "import": "./dist/index.esm.js"
    },
    "./utils": {
      "import": "./dist/utils.esm.js",
      "default": "./dist/utils.js",
      "types": "./dist/utils.d.ts"
    },
    "./helpers": {
      "types": "./dist/helpers.d.ts",
      "default": "./dist/helpers.js"
    }
  }
}"#;

    c.bench_function("sort_nested_exports", |b| {
        b.iter(|| sort_package_json(nested_exports))
    });
}

fn bench_large_dependencies(c: &mut Criterion) {
    let large_deps = r#"{
  "name": "deps-package",
  "dependencies": {
    "zod": "^3.0.0",
    "react": "^18.0.0",
    "lodash": "^4.17.21",
    "express": "^4.18.0",
    "axios": "^1.0.0",
    "typescript": "^5.0.0",
    "webpack": "^5.0.0",
    "jest": "^29.0.0",
    "eslint": "^8.0.0",
    "prettier": "^3.0.0"
  },
  "devDependencies": {
    "vitest": "^1.0.0",
    "@types/node": "^20.0.0",
    "@types/react": "^18.0.0",
    "babel-loader": "^9.0.0",
    "css-loader": "^6.0.0"
  }
}"#;

    c.bench_function("sort_large_dependencies", |b| {
        b.iter(|| sort_package_json(large_deps))
    });
}

criterion_group!(
    benches,
    bench_simple_package_json,
    bench_complex_package_json,
    bench_nested_exports,
    bench_large_dependencies
);
criterion_main!(benches);
