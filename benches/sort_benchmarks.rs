use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

const SIMPLE_PACKAGE_JSON: &str = r#"{
  "version": "1.0.0",
  "dependencies": {
    "express": "^4.17.1",
    "lodash": "^4.17.21"
  },
  "name": "test-package",
  "scripts": {
    "test": "jest",
    "build": "webpack"
  },
  "description": "A test package"
}"#;

const COMPLEX_PACKAGE_JSON: &str = r#"{
  "version": "2.5.0",
  "devDependencies": {
    "webpack": "^5.0.0",
    "typescript": "^4.5.0",
    "jest": "^27.0.0",
    "eslint": "^8.0.0",
    "@types/node": "^16.0.0"
  },
  "dependencies": {
    "react": "^18.0.0",
    "react-dom": "^18.0.0",
    "express": "^4.17.1",
    "lodash": "^4.17.21",
    "axios": "^0.24.0"
  },
  "name": "complex-package",
  "main": "dist/index.js",
  "types": "dist/index.d.ts",
  "scripts": {
    "test": "jest",
    "build": "webpack",
    "lint": "eslint src",
    "dev": "webpack serve",
    "start": "node dist/index.js"
  },
  "repository": {
    "url": "https://github.com/example/repo",
    "type": "git"
  },
  "keywords": ["test", "package", "example"],
  "author": {
    "name": "Test Author",
    "email": "author@example.com"
  },
  "license": "MIT",
  "description": "A more complex test package",
  "engines": {
    "node": ">=14.0.0",
    "npm": ">=6.0.0"
  }
}"#;

const LARGE_PACKAGE_JSON: &str = r#"{
  "version": "3.2.1",
  "private": false,
  "publishConfig": {
    "registry": "https://registry.npmjs.org/",
    "access": "public"
  },
  "peerDependencies": {
    "react": ">=16.8.0",
    "react-dom": ">=16.8.0"
  },
  "optionalDependencies": {
    "fsevents": "^2.3.2"
  },
  "devDependencies": {
    "webpack": "^5.0.0",
    "typescript": "^4.5.0",
    "jest": "^27.0.0",
    "eslint": "^8.0.0",
    "@types/react": "^18.0.0",
    "@types/react-dom": "^18.0.0",
    "@types/node": "^16.0.0",
    "babel-loader": "^8.2.0",
    "ts-loader": "^9.2.0",
    "prettier": "^2.5.0",
    "husky": "^7.0.0"
  },
  "dependencies": {
    "react": "^18.0.0",
    "react-dom": "^18.0.0",
    "express": "^4.17.1",
    "lodash": "^4.17.21",
    "axios": "^0.24.0",
    "classnames": "^2.3.1",
    "prop-types": "^15.8.0"
  },
  "name": "@scope/large-package",
  "displayName": "Large Package",
  "main": "dist/index.js",
  "module": "dist/index.esm.js",
  "types": "dist/index.d.ts",
  "files": ["dist", "README.md", "LICENSE"],
  "scripts": {
    "test": "jest",
    "test:watch": "jest --watch",
    "test:coverage": "jest --coverage",
    "build": "webpack",
    "build:prod": "webpack --mode production",
    "lint": "eslint src",
    "lint:fix": "eslint src --fix",
    "format": "prettier --write src",
    "dev": "webpack serve",
    "start": "node dist/index.js",
    "prepare": "husky install",
    "prepublishOnly": "npm run build"
  },
  "repository": {
    "type": "git",
    "url": "https://github.com/example/large-repo"
  },
  "bugs": {
    "url": "https://github.com/example/large-repo/issues",
    "email": "bugs@example.com"
  },
  "homepage": "https://example.com",
  "keywords": ["react", "component", "ui", "library", "typescript"],
  "author": {
    "name": "Test Author",
    "email": "author@example.com",
    "url": "https://example.com"
  },
  "contributors": [
    {
      "name": "Contributor One",
      "email": "one@example.com"
    },
    {
      "name": "Contributor Two",
      "email": "two@example.com"
    }
  ],
  "license": "MIT",
  "description": "A large, feature-rich test package with many fields",
  "engines": {
    "node": ">=14.0.0",
    "npm": ">=6.0.0"
  },
  "browserslist": [
    ">0.2%",
    "not dead",
    "not op_mini all"
  ],
  "prettier": {
    "semi": true,
    "singleQuote": true
  },
  "eslintConfig": {
    "extends": ["react-app"]
  },
  "jest": {
    "testEnvironment": "jsdom",
    "collectCoverageFrom": ["src/**/*.{js,jsx,ts,tsx}"]
  }
}"#;

fn bench_simple_package_json(c: &mut Criterion) {
    c.bench_function("sort simple package.json", |b| {
        b.iter(|| sort_package_json(black_box(SIMPLE_PACKAGE_JSON)))
    });
}

fn bench_complex_package_json(c: &mut Criterion) {
    c.bench_function("sort complex package.json", |b| {
        b.iter(|| sort_package_json(black_box(COMPLEX_PACKAGE_JSON)))
    });
}

fn bench_large_package_json(c: &mut Criterion) {
    c.bench_function("sort large package.json", |b| {
        b.iter(|| sort_package_json(black_box(LARGE_PACKAGE_JSON)))
    });
}

criterion_group!(
    benches,
    bench_simple_package_json,
    bench_complex_package_json,
    bench_large_package_json
);
criterion_main!(benches);
