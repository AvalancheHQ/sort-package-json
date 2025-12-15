use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

const SMALL_PACKAGE_JSON: &str = r#"{
  "version": "1.0.0",
  "dependencies": {
    "zod": "^3.0.0",
    "react": "^18.0.0",
    "axios": "^1.0.0"
  },
  "name": "my-package",
  "scripts": {
    "test": "jest",
    "build": "tsc",
    "start": "node index.js"
  },
  "description": "A small test package"
}"#;

const MEDIUM_PACKAGE_JSON: &str = r#"{
  "version": "2.1.0",
  "license": "MIT",
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "typescript": "^5.0.0",
    "zod": "^3.21.0",
    "express": "^4.18.0",
    "lodash": "^4.17.21",
    "axios": "^1.4.0",
    "moment": "^2.29.4"
  },
  "name": "@scope/medium-package",
  "devDependencies": {
    "jest": "^29.5.0",
    "eslint": "^8.40.0",
    "prettier": "^2.8.8",
    "@types/node": "^20.0.0",
    "@types/react": "^18.2.0",
    "webpack": "^5.85.0"
  },
  "scripts": {
    "test": "jest",
    "build": "webpack",
    "start": "node dist/index.js",
    "lint": "eslint src",
    "format": "prettier --write src",
    "dev": "webpack serve"
  },
  "keywords": ["react", "typescript", "webpack", "jest"],
  "author": {
    "name": "John Doe",
    "email": "john@example.com",
    "url": "https://example.com"
  },
  "repository": {
    "type": "git",
    "url": "https://github.com/example/repo"
  },
  "bugs": {
    "url": "https://github.com/example/repo/issues",
    "email": "bugs@example.com"
  },
  "homepage": "https://example.com",
  "description": "A medium-sized package with multiple fields"
}"#;

const LARGE_PACKAGE_JSON: &str = r#"{
  "version": "3.5.2",
  "license": "Apache-2.0",
  "peerDependencies": {
    "react": ">=17.0.0",
    "react-dom": ">=17.0.0"
  },
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "typescript": "^5.1.0",
    "zod": "^3.21.4",
    "express": "^4.18.2",
    "lodash": "^4.17.21",
    "axios": "^1.4.0",
    "moment": "^2.29.4",
    "redux": "^4.2.1",
    "react-redux": "^8.0.5",
    "next": "^13.4.0",
    "graphql": "^16.6.0",
    "apollo-client": "^3.7.0"
  },
  "name": "@company/large-complex-package",
  "devDependencies": {
    "jest": "^29.5.0",
    "eslint": "^8.42.0",
    "prettier": "^2.8.8",
    "@types/node": "^20.2.0",
    "@types/react": "^18.2.7",
    "webpack": "^5.85.0",
    "@typescript-eslint/parser": "^5.59.0",
    "@testing-library/react": "^14.0.0",
    "husky": "^8.0.3",
    "lint-staged": "^13.2.0"
  },
  "scripts": {
    "test": "jest --coverage",
    "test:watch": "jest --watch",
    "build": "webpack --mode production",
    "build:dev": "webpack --mode development",
    "start": "node dist/server.js",
    "start:prod": "NODE_ENV=production node dist/server.js",
    "lint": "eslint src --ext .ts,.tsx",
    "lint:fix": "eslint src --ext .ts,.tsx --fix",
    "format": "prettier --write \"src/**/*.{ts,tsx,json}\"",
    "format:check": "prettier --check \"src/**/*.{ts,tsx,json}\"",
    "dev": "webpack serve --mode development",
    "typecheck": "tsc --noEmit",
    "prepare": "husky install",
    "precommit": "lint-staged"
  },
  "keywords": [
    "react",
    "typescript",
    "webpack",
    "jest",
    "redux",
    "graphql",
    "next",
    "ssr",
    "monorepo"
  ],
  "author": {
    "name": "Jane Smith",
    "email": "jane@company.com",
    "url": "https://company.com/jane"
  },
  "contributors": [
    {
      "name": "Bob Johnson",
      "email": "bob@company.com"
    },
    {
      "name": "Alice Williams",
      "email": "alice@company.com",
      "url": "https://company.com/alice"
    }
  ],
  "repository": {
    "type": "git",
    "url": "https://github.com/company/large-package"
  },
  "bugs": {
    "url": "https://github.com/company/large-package/issues",
    "email": "support@company.com"
  },
  "homepage": "https://company.com/large-package",
  "funding": {
    "type": "opencollective",
    "url": "https://opencollective.com/large-package"
  },
  "engines": {
    "node": ">=16.0.0",
    "npm": ">=8.0.0",
    "yarn": ">=1.22.0"
  },
  "publishConfig": {
    "access": "public",
    "registry": "https://registry.npmjs.org"
  },
  "workspaces": [
    "packages/*"
  ],
  "files": [
    "dist",
    "src",
    "README.md",
    "LICENSE"
  ],
  "browserslist": [
    ">0.2%",
    "not dead",
    "not ie <= 11"
  ],
  "prettier": {
    "semi": true,
    "singleQuote": true,
    "trailingComma": "es5"
  },
  "eslintConfig": {
    "extends": ["react-app", "prettier"],
    "rules": {
      "no-console": "warn"
    }
  },
  "jest": {
    "testEnvironment": "jsdom",
    "collectCoverageFrom": ["src/**/*.{ts,tsx}"]
  },
  "lint-staged": {
    "*.{ts,tsx}": ["eslint --fix", "prettier --write"],
    "*.{json,md}": ["prettier --write"]
  },
  "description": "A large, complex package with many configuration fields and dependencies"
}"#;

fn bench_small_package(c: &mut Criterion) {
    c.bench_function("sort small package.json", |b| {
        b.iter(|| sort_package_json(black_box(SMALL_PACKAGE_JSON)))
    });
}

fn bench_medium_package(c: &mut Criterion) {
    c.bench_function("sort medium package.json", |b| {
        b.iter(|| sort_package_json(black_box(MEDIUM_PACKAGE_JSON)))
    });
}

fn bench_large_package(c: &mut Criterion) {
    c.bench_function("sort large package.json", |b| {
        b.iter(|| sort_package_json(black_box(LARGE_PACKAGE_JSON)))
    });
}

fn bench_repeated_sorting(c: &mut Criterion) {
    c.bench_function("sort already sorted package.json", |b| {
        let sorted = sort_package_json(MEDIUM_PACKAGE_JSON).unwrap();
        b.iter(|| sort_package_json(black_box(&sorted)))
    });
}

criterion_group!(
    benches,
    bench_small_package,
    bench_medium_package,
    bench_large_package,
    bench_repeated_sorting
);
criterion_main!(benches);
