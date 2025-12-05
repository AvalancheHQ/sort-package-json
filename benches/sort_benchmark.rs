use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

// Small package.json (typical size: ~500 bytes)
const SMALL_PACKAGE_JSON: &str = r#"{
  "name": "small-package",
  "version": "1.0.0",
  "scripts": {
    "test": "jest",
    "build": "webpack"
  },
  "dependencies": {
    "react": "^18.0.0"
  },
  "devDependencies": {
    "webpack": "^5.0.0"
  }
}"#;

// Medium package.json (typical size: ~2KB)
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
  "types": "./dist/index.d.ts",
  "files": ["src", "dist"],
  "scripts": {
    "test": "jest",
    "build": "webpack",
    "lint": "eslint ."
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

// Large package.json (typical monorepo size: ~5KB)
const LARGE_PACKAGE_JSON: &str = r#"{
  "name": "large-monorepo-package",
  "version": "2.5.0",
  "description": "A comprehensive monorepo package configuration",
  "keywords": ["monorepo", "workspace", "typescript", "react", "tools", "build", "test", "development"],
  "homepage": "https://github.com/example/large-package#readme",
  "bugs": {
    "email": "support@example.com",
    "url": "https://github.com/example/large-package/issues"
  },
  "repository": {
    "url": "https://github.com/example/large-package",
    "type": "git",
    "directory": "packages/core"
  },
  "license": "MIT",
  "author": {
    "name": "Example Team",
    "email": "team@example.com",
    "url": "https://example.com"
  },
  "maintainers": [
    {"name": "Lead Dev", "email": "lead@example.com"},
    {"name": "Core Contributor", "email": "contributor@example.com"}
  ],
  "contributors": [
    {"name": "Contributor 1", "email": "c1@example.com"},
    {"name": "Contributor 2", "email": "c2@example.com"}
  ],
  "funding": {
    "type": "opencollective",
    "url": "https://opencollective.com/example"
  },
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
    "./components": {
      "types": "./dist/components/index.d.ts",
      "import": "./dist/components/index.esm.js",
      "default": "./dist/components/index.js"
    }
  },
  "main": "./dist/index.js",
  "module": "./dist/index.esm.js",
  "types": "./dist/index.d.ts",
  "files": ["dist", "src", "README.md", "LICENSE", "CHANGELOG.md"],
  "workspaces": ["packages/*", "apps/*"],
  "scripts": {
    "build": "turbo run build",
    "dev": "turbo run dev --parallel",
    "test": "turbo run test",
    "test:watch": "jest --watch",
    "test:coverage": "jest --coverage",
    "lint": "eslint . --ext .ts,.tsx,.js,.jsx",
    "lint:fix": "eslint . --ext .ts,.tsx,.js,.jsx --fix",
    "format": "prettier --write \"**/*.{ts,tsx,js,jsx,json,md}\"",
    "format:check": "prettier --check \"**/*.{ts,tsx,js,jsx,json,md}\"",
    "typecheck": "tsc --noEmit",
    "clean": "rimraf dist coverage .turbo",
    "prepare": "husky install",
    "precommit": "lint-staged",
    "prepublishOnly": "npm run build && npm run test",
    "release": "changeset publish"
  },
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "axios": "^1.4.0",
    "lodash": "^4.17.21",
    "date-fns": "^2.30.0",
    "classnames": "^2.3.2",
    "zustand": "^4.3.8"
  },
  "devDependencies": {
    "@types/react": "^18.2.0",
    "@types/react-dom": "^18.2.0",
    "@types/node": "^20.0.0",
    "@types/lodash": "^4.14.195",
    "typescript": "^5.1.0",
    "webpack": "^5.88.0",
    "webpack-cli": "^5.1.4",
    "webpack-dev-server": "^4.15.0",
    "babel-loader": "^9.1.2",
    "@babel/core": "^7.22.0",
    "@babel/preset-env": "^7.22.0",
    "@babel/preset-react": "^7.22.0",
    "@babel/preset-typescript": "^7.22.0",
    "jest": "^29.5.0",
    "@testing-library/react": "^14.0.0",
    "@testing-library/jest-dom": "^5.16.5",
    "eslint": "^8.42.0",
    "@typescript-eslint/parser": "^5.59.0",
    "@typescript-eslint/eslint-plugin": "^5.59.0",
    "eslint-config-prettier": "^8.8.0",
    "prettier": "^2.8.8",
    "husky": "^8.0.3",
    "lint-staged": "^13.2.2",
    "turbo": "^1.10.0",
    "rimraf": "^5.0.1"
  },
  "peerDependencies": {
    "react": ">=18.0.0",
    "react-dom": ">=18.0.0"
  },
  "engines": {
    "node": ">=18.0.0",
    "npm": ">=9.0.0"
  },
  "packageManager": "npm@9.6.7",
  "publishConfig": {
    "access": "public",
    "registry": "https://registry.npmjs.org/"
  },
  "eslintConfig": {
    "extends": ["eslint:recommended", "plugin:@typescript-eslint/recommended", "prettier"],
    "parser": "@typescript-eslint/parser",
    "plugins": ["@typescript-eslint"]
  },
  "prettier": {
    "semi": true,
    "singleQuote": true,
    "trailingComma": "es5",
    "printWidth": 100
  },
  "lint-staged": {
    "*.{ts,tsx,js,jsx}": ["eslint --fix", "prettier --write"],
    "*.{json,md}": ["prettier --write"]
  },
  "babel": {
    "presets": ["@babel/preset-env", "@babel/preset-react", "@babel/preset-typescript"],
    "plugins": []
  }
}"#;

fn bench_small_package(c: &mut Criterion) {
    c.bench_function("sort_small_package", |b| {
        b.iter(|| sort_package_json(black_box(SMALL_PACKAGE_JSON)))
    });
}

fn bench_medium_package(c: &mut Criterion) {
    c.bench_function("sort_medium_package", |b| {
        b.iter(|| sort_package_json(black_box(MEDIUM_PACKAGE_JSON)))
    });
}

fn bench_large_package(c: &mut Criterion) {
    c.bench_function("sort_large_package", |b| {
        b.iter(|| sort_package_json(black_box(LARGE_PACKAGE_JSON)))
    });
}

fn bench_already_sorted(c: &mut Criterion) {
    // Pre-sort the medium package to test idempotency performance
    let sorted = sort_package_json(MEDIUM_PACKAGE_JSON)
        .expect("Failed to sort package.json for benchmark");
    
    c.bench_function("sort_already_sorted", |b| {
        b.iter(|| sort_package_json(black_box(&sorted)))
    });
}

criterion_group!(
    benches,
    bench_small_package,
    bench_medium_package,
    bench_large_package,
    bench_already_sorted
);
criterion_main!(benches);
