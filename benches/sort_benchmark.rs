use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

const SAMPLE_PACKAGE_JSON: &str = r#"{
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

const MINIMAL_PACKAGE_JSON: &str = r#"{
  "name": "minimal-package",
  "version": "1.0.0",
  "description": "A minimal package.json"
}"#;

const LARGE_PACKAGE_JSON: &str = r#"{
  "name": "large-package",
  "version": "2.5.0",
  "description": "A large package with many dependencies",
  "keywords": ["api", "framework", "web", "server", "http", "express", "middleware", "router"],
  "homepage": "https://github.com/example/large-package",
  "bugs": {
    "url": "https://github.com/example/large-package/issues"
  },
  "repository": {
    "type": "git",
    "url": "https://github.com/example/large-package"
  },
  "license": "MIT",
  "author": "Example Team",
  "main": "./dist/index.js",
  "types": "./dist/index.d.ts",
  "files": ["dist", "src", "LICENSE", "README.md"],
  "scripts": {
    "build": "tsc",
    "test": "jest",
    "lint": "eslint src",
    "format": "prettier --write src",
    "prepublishOnly": "npm run build",
    "dev": "tsc --watch"
  },
  "dependencies": {
    "@types/node": "^20.0.0",
    "express": "^4.18.0",
    "lodash": "^4.17.21",
    "axios": "^1.4.0",
    "dotenv": "^16.0.0",
    "uuid": "^9.0.0",
    "bcrypt": "^5.1.0",
    "jsonwebtoken": "^9.0.0",
    "winston": "^3.8.0",
    "morgan": "^1.10.0",
    "helmet": "^7.0.0",
    "cors": "^2.8.5",
    "compression": "^1.7.4",
    "body-parser": "^1.20.0",
    "cookie-parser": "^1.4.6",
    "multer": "^1.4.5"
  },
  "devDependencies": {
    "@types/express": "^4.17.17",
    "@types/jest": "^29.5.0",
    "@types/bcrypt": "^5.0.0",
    "@types/jsonwebtoken": "^9.0.0",
    "@types/morgan": "^1.9.4",
    "@types/cors": "^2.8.13",
    "@types/compression": "^1.7.2",
    "@types/cookie-parser": "^1.4.3",
    "@types/multer": "^1.4.7",
    "@typescript-eslint/eslint-plugin": "^5.59.0",
    "@typescript-eslint/parser": "^5.59.0",
    "eslint": "^8.40.0",
    "jest": "^29.5.0",
    "prettier": "^2.8.8",
    "ts-jest": "^29.1.0",
    "typescript": "^5.0.4"
  },
  "engines": {
    "node": ">=18.0.0",
    "npm": ">=9.0.0"
  }
}"#;

fn bench_sort_typical(c: &mut Criterion) {
    c.bench_function("sort typical package.json", |b| {
        b.iter(|| sort_package_json(black_box(SAMPLE_PACKAGE_JSON)))
    });
}

fn bench_sort_minimal(c: &mut Criterion) {
    c.bench_function("sort minimal package.json", |b| {
        b.iter(|| sort_package_json(black_box(MINIMAL_PACKAGE_JSON)))
    });
}

fn bench_sort_large(c: &mut Criterion) {
    c.bench_function("sort large package.json", |b| {
        b.iter(|| sort_package_json(black_box(LARGE_PACKAGE_JSON)))
    });
}

fn bench_sort_idempotent(c: &mut Criterion) {
    let sorted_once = sort_package_json(SAMPLE_PACKAGE_JSON).unwrap();
    c.bench_function("sort already sorted package.json", |b| {
        b.iter(|| sort_package_json(black_box(&sorted_once)))
    });
}

criterion_group!(
    benches,
    bench_sort_typical,
    bench_sort_minimal,
    bench_sort_large,
    bench_sort_idempotent
);
criterion_main!(benches);
