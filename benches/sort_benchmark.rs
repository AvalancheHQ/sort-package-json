use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

const SIMPLE_PACKAGE_JSON: &str = r#"{
  "version": "1.0.0",
  "dependencies": {
    "lodash": "^4.17.21"
  },
  "name": "simple-package",
  "scripts": {
    "test": "jest"
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

const LARGE_DEPENDENCIES: &str = r#"{
  "name": "large-package",
  "version": "1.0.0",
  "dependencies": {
    "react": "^18.0.0",
    "react-dom": "^18.0.0",
    "axios": "^1.0.0",
    "lodash": "^4.17.21",
    "moment": "^2.29.4",
    "express": "^4.18.2",
    "webpack": "^5.88.0",
    "typescript": "^5.0.0",
    "jest": "^29.0.0",
    "eslint": "^8.0.0",
    "@babel/core": "^7.22.0",
    "@babel/preset-env": "^7.22.0",
    "postcss": "^8.4.0",
    "autoprefixer": "^10.4.0",
    "tailwindcss": "^3.3.0"
  },
  "devDependencies": {
    "@types/node": "^20.0.0",
    "@types/react": "^18.0.0",
    "@typescript-eslint/parser": "^5.0.0",
    "prettier": "^3.0.0",
    "husky": "^8.0.0"
  },
  "scripts": {
    "start": "node index.js",
    "dev": "nodemon index.js",
    "build": "webpack --mode production",
    "test": "jest",
    "lint": "eslint .",
    "format": "prettier --write ."
  }
}"#;

fn bench_simple_package(c: &mut Criterion) {
    c.bench_function("sort_simple_package", |b| {
        b.iter(|| {
            sort_package_json(black_box(SIMPLE_PACKAGE_JSON))
                .expect("Failed to sort simple package.json")
        })
    });
}

fn bench_complex_package(c: &mut Criterion) {
    c.bench_function("sort_complex_package", |b| {
        b.iter(|| {
            sort_package_json(black_box(COMPLEX_PACKAGE_JSON))
                .expect("Failed to sort complex package.json")
        })
    });
}

fn bench_large_dependencies(c: &mut Criterion) {
    c.bench_function("sort_large_dependencies", |b| {
        b.iter(|| {
            sort_package_json(black_box(LARGE_DEPENDENCIES))
                .expect("Failed to sort large dependencies package.json")
        })
    });
}

fn bench_idempotency(c: &mut Criterion) {
    let sorted_once =
        sort_package_json(COMPLEX_PACKAGE_JSON).expect("Failed to sort for idempotency test");

    c.bench_function("sort_idempotency", |b| {
        b.iter(|| {
            sort_package_json(black_box(&sorted_once))
                .expect("Failed to sort already sorted package.json")
        })
    });
}

criterion_group!(
    benches,
    bench_simple_package,
    bench_complex_package,
    bench_large_dependencies,
    bench_idempotency
);
criterion_main!(benches);
