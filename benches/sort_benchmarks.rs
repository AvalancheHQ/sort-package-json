use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

fn bench_small_package_json(c: &mut Criterion) {
    let small_json = r#"{
  "name": "small-package",
  "version": "1.0.0",
  "dependencies": {
    "react": "^18.0.0"
  }
}"#;

    c.bench_function("sort_small_package_json", |b| {
        b.iter(|| sort_package_json(small_json))
    });
}

fn bench_medium_package_json(c: &mut Criterion) {
    let medium_json = r#"{
  "name": "medium-package",
  "version": "1.0.0",
  "description": "A medium-sized package for testing",
  "keywords": ["test", "benchmark", "json", "sorting"],
  "homepage": "https://github.com/test/medium-package",
  "bugs": {
    "url": "https://github.com/test/medium-package/issues",
    "email": "bugs@example.com"
  },
  "repository": {
    "type": "git",
    "url": "https://github.com/test/medium-package"
  },
  "license": "MIT",
  "author": {
    "name": "Test Author",
    "email": "author@example.com"
  },
  "scripts": {
    "test": "jest",
    "build": "webpack",
    "lint": "eslint .",
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
    "node": ">=18.0.0",
    "npm": ">=8.0.0"
  }
}"#;

    c.bench_function("sort_medium_package_json", |b| {
        b.iter(|| sort_package_json(medium_json))
    });
}

fn bench_large_package_json(c: &mut Criterion) {
    let large_json = r#"{
  "name": "large-package",
  "version": "2.5.0",
  "description": "A large package with many fields for comprehensive benchmarking",
  "keywords": ["test", "benchmark", "json", "sorting", "performance", "monorepo"],
  "homepage": "https://github.com/test/large-package#readme",
  "bugs": {
    "url": "https://github.com/test/large-package/issues",
    "email": "bugs@example.com"
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
      "email": "m1@example.com",
      "url": "https://m1.example.com"
    },
    {
      "name": "Maintainer Two",
      "email": "m2@example.com",
      "url": "https://m2.example.com"
    }
  ],
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
  "files": [
    "src",
    "dist",
    "README.md",
    "LICENSE"
  ],
  "workspaces": [
    "packages/*"
  ],
  "scripts": {
    "test": "jest",
    "pretest": "echo 'Starting tests'",
    "posttest": "echo 'Tests complete'",
    "build": "webpack",
    "prebuild": "npm run clean",
    "postbuild": "npm run types",
    "lint": "eslint .",
    "prelint": "npm run format:check",
    "format": "prettier --write .",
    "format:check": "prettier --check .",
    "types": "tsc",
    "dev": "webpack serve",
    "clean": "rimraf dist"
  },
  "husky": {
    "hooks": {
      "pre-commit": "lint-staged",
      "commit-msg": "commitlint -E HUSKY_GIT_PARAMS"
    }
  },
  "lint-staged": {
    "*.{js,jsx,ts,tsx}": ["eslint --fix", "prettier --write"],
    "*.{json,md}": ["prettier --write"]
  },
  "prettier": {
    "semi": true,
    "singleQuote": true,
    "trailingComma": "es5"
  },
  "eslintConfig": {
    "extends": ["eslint:recommended", "plugin:react/recommended"],
    "rules": {
      "no-console": "warn"
    }
  },
  "babel": {
    "presets": ["@babel/preset-env", "@babel/preset-react", "@babel/preset-typescript"],
    "plugins": ["@babel/plugin-proposal-class-properties", "@babel/plugin-transform-runtime"]
  },
  "jest": {
    "testEnvironment": "jsdom",
    "setupFilesAfterEnv": ["<rootDir>/jest.setup.js"],
    "collectCoverageFrom": ["src/**/*.{js,jsx,ts,tsx}"]
  },
  "resolutions": {
    "minimist": "^1.2.6",
    "ansi-regex": "^5.0.1"
  },
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "axios": "^1.4.0",
    "lodash": "^4.17.21",
    "clsx": "^2.0.0",
    "date-fns": "^2.30.0"
  },
  "devDependencies": {
    "webpack": "^5.88.0",
    "webpack-cli": "^5.1.4",
    "webpack-dev-server": "^4.15.1",
    "jest": "^29.6.0",
    "@testing-library/react": "^14.0.0",
    "@testing-library/jest-dom": "^5.16.5",
    "typescript": "^5.1.6",
    "@types/react": "^18.2.15",
    "@types/react-dom": "^18.2.7",
    "@types/lodash": "^4.14.195",
    "eslint": "^8.45.0",
    "prettier": "^3.0.0",
    "husky": "^8.0.3",
    "lint-staged": "^13.2.3",
    "rimraf": "^5.0.1"
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
  "customField": "this is a custom field",
  "_internal": "private internal field",
  "_metadata": "private metadata"
}"#;

    c.bench_function("sort_large_package_json", |b| {
        b.iter(|| sort_package_json(large_json))
    });
}

fn bench_idempotency(c: &mut Criterion) {
    let json = r#"{
  "name": "test-package",
  "version": "1.0.0",
  "scripts": {
    "test": "jest",
    "build": "webpack"
  },
  "dependencies": {
    "react": "^18.0.0"
  }
}"#;

    let sorted_once = sort_package_json(json).expect("First sort failed");

    c.bench_function("sort_already_sorted", |b| {
        b.iter(|| sort_package_json(&sorted_once))
    });
}

fn bench_complex_exports(c: &mut Criterion) {
    let complex_exports_json = r#"{
  "name": "exports-test",
  "version": "1.0.0",
  "exports": {
    ".": {
      "types": "./index.d.ts",
      "node": {
        "import": "./index.mjs",
        "require": "./index.cjs"
      },
      "browser": "./browser.js",
      "default": "./index.js"
    },
    "./feature": {
      "types": "./feature.d.ts",
      "import": "./feature.mjs",
      "require": "./feature.cjs"
    },
    "./utils/*": {
      "types": "./utils/*.d.ts",
      "default": "./utils/*.js"
    }
  }
}"#;

    c.bench_function("sort_complex_exports", |b| {
        b.iter(|| sort_package_json(complex_exports_json))
    });
}

fn bench_many_dependencies(c: &mut Criterion) {
    let many_deps_json = r#"{
  "name": "deps-heavy",
  "version": "1.0.0",
  "dependencies": {
    "react": "^18.0.0",
    "react-dom": "^18.0.0",
    "axios": "^1.0.0",
    "lodash": "^4.17.21",
    "moment": "^2.29.4",
    "express": "^4.18.2",
    "mongoose": "^7.0.0",
    "joi": "^17.9.0",
    "bcrypt": "^5.1.0",
    "jsonwebtoken": "^9.0.0",
    "dotenv": "^16.0.3",
    "cors": "^2.8.5",
    "helmet": "^7.0.0",
    "winston": "^3.8.2",
    "nodemon": "^2.0.22"
  },
  "devDependencies": {
    "jest": "^29.0.0",
    "eslint": "^8.0.0",
    "prettier": "^3.0.0",
    "typescript": "^5.0.0",
    "@types/node": "^20.0.0",
    "@types/express": "^4.17.17",
    "@types/cors": "^2.8.13",
    "@types/bcrypt": "^5.0.0",
    "@types/jsonwebtoken": "^9.0.2"
  }
}"#;

    c.bench_function("sort_many_dependencies", |b| {
        b.iter(|| sort_package_json(many_deps_json))
    });
}

criterion_group!(
    benches,
    bench_small_package_json,
    bench_medium_package_json,
    bench_large_package_json,
    bench_idempotency,
    bench_complex_exports,
    bench_many_dependencies
);
criterion_main!(benches);
