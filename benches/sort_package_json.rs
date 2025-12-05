use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

fn benchmark_small_package_json(c: &mut Criterion) {
    let input = r#"{
  "version": "1.0.0",
  "dependencies": {
    "react": "^18.0.0",
    "typescript": "^5.0.0"
  },
  "name": "my-package",
  "scripts": {
    "build": "tsc",
    "test": "jest"
  },
  "description": "A sample package"
}"#;

    c.bench_function("sort_small_package_json", |b| {
        b.iter(|| sort_package_json(black_box(input)))
    });
}

fn benchmark_medium_package_json(c: &mut Criterion) {
    let input = r#"{
  "version": "2.5.1",
  "devDependencies": {
    "webpack": "^5.0.0",
    "babel-loader": "^9.0.0",
    "eslint": "^8.0.0",
    "jest": "^29.0.0",
    "prettier": "^3.0.0",
    "typescript": "^5.0.0",
    "@types/react": "^18.0.0",
    "@types/node": "^20.0.0"
  },
  "name": "medium-package",
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "lodash": "^4.17.21",
    "axios": "^1.4.0",
    "express": "^4.18.2",
    "moment": "^2.29.4"
  },
  "scripts": {
    "start": "node server.js",
    "build": "webpack --mode production",
    "test": "jest",
    "lint": "eslint src",
    "format": "prettier --write .",
    "dev": "webpack serve --mode development"
  },
  "description": "A medium-sized package with multiple dependencies",
  "keywords": ["react", "webpack", "typescript"],
  "author": {
    "name": "John Doe",
    "email": "john@example.com",
    "url": "https://example.com"
  },
  "license": "MIT",
  "repository": {
    "type": "git",
    "url": "https://github.com/example/repo"
  },
  "bugs": {
    "url": "https://github.com/example/repo/issues",
    "email": "bugs@example.com"
  },
  "homepage": "https://example.com",
  "engines": {
    "node": ">=18.0.0",
    "npm": ">=9.0.0"
  }
}"#;

    c.bench_function("sort_medium_package_json", |b| {
        b.iter(|| sort_package_json(black_box(input)))
    });
}

fn benchmark_large_package_json(c: &mut Criterion) {
    let input = r#"{
  "$schema": "https://json.schemastore.org/package.json",
  "version": "3.2.1",
  "private": false,
  "devDependencies": {
    "webpack": "^5.88.0",
    "webpack-cli": "^5.1.4",
    "webpack-dev-server": "^4.15.1",
    "babel-loader": "^9.1.2",
    "@babel/core": "^7.22.5",
    "@babel/preset-env": "^7.22.5",
    "@babel/preset-react": "^7.22.5",
    "@babel/preset-typescript": "^7.22.5",
    "eslint": "^8.43.0",
    "eslint-config-prettier": "^8.8.0",
    "eslint-plugin-react": "^7.32.2",
    "jest": "^29.5.0",
    "@testing-library/react": "^14.0.0",
    "@testing-library/jest-dom": "^5.16.5",
    "prettier": "^3.0.0",
    "typescript": "^5.1.3",
    "@types/react": "^18.2.14",
    "@types/react-dom": "^18.2.6",
    "@types/node": "^20.3.1",
    "@types/lodash": "^4.14.195",
    "husky": "^8.0.3",
    "lint-staged": "^13.2.2",
    "nodemon": "^2.0.22"
  },
  "name": "large-complex-package",
  "displayName": "Large Complex Package",
  "description": "A comprehensive package with many dependencies and configurations",
  "keywords": ["react", "webpack", "typescript", "eslint", "jest", "babel"],
  "categories": ["frameworks", "testing", "linters"],
  "homepage": "https://example.com",
  "bugs": {
    "url": "https://github.com/example/large-repo/issues",
    "email": "bugs@example.com"
  },
  "repository": {
    "type": "git",
    "url": "https://github.com/example/large-repo"
  },
  "funding": {
    "type": "opencollective",
    "url": "https://opencollective.com/example"
  },
  "license": "MIT",
  "author": {
    "name": "Jane Smith",
    "email": "jane@example.com",
    "url": "https://janesmith.dev"
  },
  "contributors": [
    {
      "name": "John Doe",
      "email": "john@example.com"
    },
    {
      "name": "Alice Johnson",
      "email": "alice@example.com"
    }
  ],
  "type": "module",
  "main": "./dist/index.js",
  "module": "./dist/index.mjs",
  "types": "./dist/index.d.ts",
  "exports": {
    ".": {
      "types": "./dist/index.d.ts",
      "import": "./dist/index.mjs",
      "require": "./dist/index.js"
    },
    "./utils": {
      "types": "./dist/utils.d.ts",
      "import": "./dist/utils.mjs",
      "require": "./dist/utils.js"
    }
  },
  "files": ["dist", "README.md", "LICENSE"],
  "bin": {
    "my-cli": "./dist/cli.js"
  },
  "scripts": {
    "start": "node server.js",
    "dev": "webpack serve --mode development",
    "build": "webpack --mode production",
    "build:types": "tsc --emitDeclarationOnly",
    "test": "jest",
    "test:watch": "jest --watch",
    "test:coverage": "jest --coverage",
    "lint": "eslint src --ext .ts,.tsx",
    "lint:fix": "eslint src --ext .ts,.tsx --fix",
    "format": "prettier --write .",
    "format:check": "prettier --check .",
    "typecheck": "tsc --noEmit",
    "prepare": "husky install",
    "prepublishOnly": "npm run build"
  },
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "lodash": "^4.17.21",
    "axios": "^1.4.0",
    "express": "^4.18.2",
    "moment": "^2.29.4",
    "redux": "^4.2.1",
    "react-redux": "^8.1.1",
    "react-router-dom": "^6.14.0",
    "styled-components": "^6.0.4"
  },
  "peerDependencies": {
    "react": ">=18.0.0",
    "react-dom": ">=18.0.0"
  },
  "optionalDependencies": {
    "fsevents": "^2.3.2"
  },
  "engines": {
    "node": ">=18.0.0",
    "npm": ">=9.0.0"
  },
  "packageManager": "npm@9.8.0",
  "browserslist": [
    ">0.2%",
    "not dead",
    "not op_mini all"
  ],
  "eslintConfig": {
    "extends": ["react-app", "prettier"],
    "rules": {
      "no-console": "warn"
    }
  },
  "prettier": {
    "semi": true,
    "singleQuote": true,
    "tabWidth": 2
  },
  "jest": {
    "testEnvironment": "jsdom",
    "setupFilesAfterEnv": ["<rootDir>/jest.setup.js"]
  },
  "lint-staged": {
    "*.{ts,tsx}": ["eslint --fix", "prettier --write"],
    "*.{json,md}": ["prettier --write"]
  },
  "_customField": "value",
  "_privateConfig": {
    "internal": true
  }
}"#;

    c.bench_function("sort_large_package_json", |b| {
        b.iter(|| sort_package_json(black_box(input)))
    });
}

fn benchmark_already_sorted(c: &mut Criterion) {
    let input = r#"{
  "name": "sorted-package",
  "version": "1.0.0",
  "description": "Already sorted",
  "scripts": {
    "build": "tsc",
    "test": "jest"
  },
  "dependencies": {
    "react": "^18.0.0"
  }
}"#;

    c.bench_function("sort_already_sorted_package_json", |b| {
        b.iter(|| sort_package_json(black_box(input)))
    });
}

criterion_group!(
    benches,
    benchmark_small_package_json,
    benchmark_medium_package_json,
    benchmark_large_package_json,
    benchmark_already_sorted
);
criterion_main!(benches);
