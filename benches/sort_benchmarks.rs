use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

const SIMPLE_PACKAGE_JSON: &str = r#"{
  "version": "1.0.0",
  "dependencies": {
    "express": "^4.18.0",
    "react": "^18.0.0",
    "lodash": "^4.17.21"
  },
  "name": "my-package",
  "scripts": {
    "test": "jest",
    "build": "webpack",
    "start": "node index.js"
  },
  "description": "A sample package"
}"#;

const COMPLEX_PACKAGE_JSON: &str = r#"{
  "version": "2.5.0",
  "keywords": ["utility", "tool", "npm"],
  "dependencies": {
    "express": "^4.18.0",
    "react": "^18.0.0",
    "lodash": "^4.17.21",
    "axios": "^1.0.0",
    "dotenv": "^16.0.0"
  },
  "name": "complex-package",
  "scripts": {
    "test": "jest",
    "build": "webpack",
    "start": "node index.js",
    "lint": "eslint .",
    "format": "prettier --write ."
  },
  "devDependencies": {
    "jest": "^29.0.0",
    "webpack": "^5.0.0",
    "eslint": "^8.0.0",
    "prettier": "^3.0.0"
  },
  "description": "A more complex package with many fields",
  "author": {
    "name": "John Doe",
    "email": "john@example.com",
    "url": "https://example.com"
  },
  "repository": {
    "type": "git",
    "url": "https://github.com/example/repo"
  },
  "license": "MIT",
  "engines": {
    "node": ">=18.0.0",
    "npm": ">=9.0.0"
  },
  "bugs": {
    "url": "https://github.com/example/repo/issues",
    "email": "bugs@example.com"
  },
  "homepage": "https://example.com"
}"#;

const LARGE_PACKAGE_JSON: &str = r#"{
  "version": "3.0.0",
  "keywords": ["framework", "library", "typescript", "javascript", "nodejs", "web", "api"],
  "dependencies": {
    "express": "^4.18.0",
    "react": "^18.0.0",
    "lodash": "^4.17.21",
    "axios": "^1.0.0",
    "dotenv": "^16.0.0",
    "bcrypt": "^5.0.0",
    "jsonwebtoken": "^9.0.0",
    "mongoose": "^7.0.0",
    "redis": "^4.0.0",
    "winston": "^3.0.0"
  },
  "name": "large-package",
  "scripts": {
    "test": "jest --coverage",
    "test:watch": "jest --watch",
    "test:e2e": "jest --config=jest-e2e.json",
    "build": "webpack --mode production",
    "build:dev": "webpack --mode development",
    "start": "node dist/index.js",
    "start:dev": "nodemon src/index.js",
    "lint": "eslint . --ext .js,.jsx,.ts,.tsx",
    "lint:fix": "eslint . --ext .js,.jsx,.ts,.tsx --fix",
    "format": "prettier --write .",
    "format:check": "prettier --check .",
    "prepare": "husky install",
    "prepublishOnly": "npm run build"
  },
  "devDependencies": {
    "jest": "^29.0.0",
    "webpack": "^5.0.0",
    "eslint": "^8.0.0",
    "prettier": "^3.0.0",
    "husky": "^8.0.0",
    "lint-staged": "^13.0.0",
    "nodemon": "^3.0.0",
    "@types/node": "^20.0.0",
    "@types/express": "^4.17.0",
    "typescript": "^5.0.0"
  },
  "peerDependencies": {
    "react": ">=18.0.0"
  },
  "optionalDependencies": {
    "fsevents": "^2.3.0"
  },
  "description": "A large package with extensive configuration",
  "author": {
    "name": "Jane Smith",
    "email": "jane@example.com",
    "url": "https://janesmith.dev"
  },
  "contributors": [
    {
      "name": "Bob Johnson",
      "email": "bob@example.com"
    },
    {
      "name": "Alice Williams",
      "email": "alice@example.com"
    }
  ],
  "repository": {
    "type": "git",
    "url": "https://github.com/example/large-repo"
  },
  "license": "MIT",
  "engines": {
    "node": ">=18.0.0",
    "npm": ">=9.0.0"
  },
  "bugs": {
    "url": "https://github.com/example/large-repo/issues",
    "email": "bugs@example.com"
  },
  "homepage": "https://example.com/large-package",
  "prettier": {
    "semi": true,
    "singleQuote": true,
    "tabWidth": 2
  },
  "eslintConfig": {
    "extends": ["eslint:recommended"],
    "env": {
      "node": true,
      "es6": true
    }
  },
  "lint-staged": {
    "*.js": ["eslint --fix", "prettier --write"],
    "*.json": ["prettier --write"]
  },
  "husky": {
    "hooks": {
      "pre-commit": "lint-staged"
    }
  }
}"#;

fn bench_simple_package(c: &mut Criterion) {
    c.bench_function("sort_simple_package", |b| {
        b.iter(|| {
            sort_package_json(black_box(SIMPLE_PACKAGE_JSON)).expect("Failed to sort package.json")
        })
    });
}

fn bench_complex_package(c: &mut Criterion) {
    c.bench_function("sort_complex_package", |b| {
        b.iter(|| {
            sort_package_json(black_box(COMPLEX_PACKAGE_JSON))
                .expect("Failed to sort package.json")
        })
    });
}

fn bench_large_package(c: &mut Criterion) {
    c.bench_function("sort_large_package", |b| {
        b.iter(|| {
            sort_package_json(black_box(LARGE_PACKAGE_JSON)).expect("Failed to sort package.json")
        })
    });
}

fn bench_repeated_sorting(c: &mut Criterion) {
    c.bench_function("sort_idempotency", |b| {
        b.iter(|| {
            let first_sort = sort_package_json(black_box(COMPLEX_PACKAGE_JSON))
                .expect("Failed first sort");
            sort_package_json(black_box(&first_sort)).expect("Failed second sort")
        })
    });
}

criterion_group!(
    benches,
    bench_simple_package,
    bench_complex_package,
    bench_large_package,
    bench_repeated_sorting
);
criterion_main!(benches);
