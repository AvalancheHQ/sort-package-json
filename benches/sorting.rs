use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

// Small package.json with basic fields (common case)
const SMALL_PACKAGE: &str = r#"{
  "version": "1.0.0",
  "dependencies": {
    "react": "^18.0.0",
    "lodash": "^4.17.21"
  },
  "name": "simple-package",
  "scripts": {
    "test": "jest",
    "build": "webpack"
  }
}"#;

// Medium package.json with more fields
const MEDIUM_PACKAGE: &str = r#"{
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
    },
    "./package.json": "./package.json"
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
  }
}"#;

// Large package.json (typical monorepo package)
const LARGE_PACKAGE: &str = r#"{
  "name": "large-monorepo-package",
  "version": "2.5.0",
  "description": "A large package with many fields",
  "keywords": ["monorepo", "workspace", "tooling", "build", "test", "benchmark"],
  "homepage": "https://github.com/org/large-package#readme",
  "bugs": {
    "url": "https://github.com/org/large-package/issues",
    "email": "bugs@example.org"
  },
  "repository": {
    "type": "git",
    "url": "https://github.com/org/large-package.git"
  },
  "funding": {
    "type": "opencollective",
    "url": "https://opencollective.com/large-package"
  },
  "license": "MIT",
  "author": {
    "name": "Package Author",
    "email": "author@example.org",
    "url": "https://author.example.org"
  },
  "maintainers": [
    {
      "name": "Maintainer One",
      "email": "m1@example.org",
      "url": "https://m1.example.org"
    },
    {
      "name": "Maintainer Two",
      "email": "m2@example.org",
      "url": "https://m2.example.org"
    }
  ],
  "contributors": [
    {
      "name": "Contributor One",
      "email": "c1@example.org"
    }
  ],
  "type": "module",
  "exports": {
    ".": {
      "types": "./dist/index.d.ts",
      "import": "./dist/index.esm.js",
      "require": "./dist/index.cjs",
      "default": "./dist/index.js"
    },
    "./cli": {
      "types": "./dist/cli.d.ts",
      "import": "./dist/cli.esm.js",
      "default": "./dist/cli.js"
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
  "bin": {
    "large-pkg": "./bin/cli.js",
    "lpkg": "./bin/cli.js"
  },
  "files": [
    "bin",
    "dist",
    "src",
    "README.md",
    "LICENSE",
    "CHANGELOG.md"
  ],
  "workspaces": [
    "packages/*",
    "tools/*"
  ],
  "scripts": {
    "build": "tsc && webpack",
    "build:watch": "tsc --watch",
    "clean": "rimraf dist coverage",
    "dev": "webpack serve --mode development",
    "format": "prettier --write .",
    "format:check": "prettier --check .",
    "lint": "eslint . --ext .ts,.tsx,.js,.jsx",
    "lint:fix": "eslint . --ext .ts,.tsx,.js,.jsx --fix",
    "prebuild": "npm run clean",
    "pretest": "npm run lint",
    "posttest": "npm run coverage:report",
    "test": "jest --coverage",
    "test:watch": "jest --watch",
    "test:ci": "jest --ci --coverage --maxWorkers=2",
    "typecheck": "tsc --noEmit",
    "prepare": "husky install",
    "prepublishOnly": "npm run build && npm test"
  },
  "dependencies": {
    "@babel/runtime": "^7.22.0",
    "axios": "^1.4.0",
    "commander": "^11.0.0",
    "dotenv": "^16.3.0",
    "lodash": "^4.17.21",
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "tslib": "^2.6.0",
    "zod": "^3.21.0"
  },
  "devDependencies": {
    "@babel/core": "^7.22.0",
    "@babel/preset-env": "^7.22.0",
    "@babel/preset-react": "^7.22.0",
    "@babel/preset-typescript": "^7.22.0",
    "@types/jest": "^29.5.0",
    "@types/node": "^20.4.0",
    "@types/react": "^18.2.0",
    "@types/react-dom": "^18.2.0",
    "@typescript-eslint/eslint-plugin": "^6.0.0",
    "@typescript-eslint/parser": "^6.0.0",
    "eslint": "^8.45.0",
    "eslint-config-prettier": "^8.8.0",
    "eslint-plugin-react": "^7.32.0",
    "eslint-plugin-react-hooks": "^4.6.0",
    "husky": "^8.0.0",
    "jest": "^29.6.0",
    "lint-staged": "^13.2.0",
    "prettier": "^3.0.0",
    "rimraf": "^5.0.0",
    "ts-jest": "^29.1.0",
    "typescript": "^5.1.0",
    "webpack": "^5.88.0",
    "webpack-cli": "^5.1.0",
    "webpack-dev-server": "^4.15.0"
  },
  "peerDependencies": {
    "react": "^18.0.0"
  },
  "engines": {
    "node": ">=18.0.0",
    "npm": ">=9.0.0"
  },
  "packageManager": "npm@9.8.0",
  "volta": {
    "node": "18.17.0",
    "npm": "9.8.0"
  },
  "publishConfig": {
    "access": "public",
    "registry": "https://registry.npmjs.org/"
  },
  "babel": {
    "presets": [
      "@babel/preset-env",
      "@babel/preset-react",
      "@babel/preset-typescript"
    ],
    "plugins": [
      "@babel/plugin-transform-runtime"
    ]
  },
  "eslintConfig": {
    "extends": [
      "eslint:recommended",
      "plugin:@typescript-eslint/recommended",
      "plugin:react/recommended",
      "prettier"
    ],
    "parser": "@typescript-eslint/parser",
    "plugins": [
      "@typescript-eslint",
      "react",
      "react-hooks"
    ],
    "rules": {
      "react/react-in-jsx-scope": "off"
    }
  },
  "prettier": {
    "semi": true,
    "singleQuote": true,
    "tabWidth": 2,
    "trailingComma": "es5"
  },
  "jest": {
    "preset": "ts-jest",
    "testEnvironment": "node",
    "coverageDirectory": "coverage",
    "collectCoverageFrom": [
      "src/**/*.{ts,tsx}",
      "!src/**/*.d.ts"
    ]
  },
  "lint-staged": {
    "*.{ts,tsx,js,jsx}": [
      "eslint --fix",
      "prettier --write"
    ],
    "*.{json,md}": [
      "prettier --write"
    ]
  },
  "customField": "custom value",
  "anotherCustom": "another custom",
  "_internal": "private field",
  "_metadata": {
    "generated": true
  }
}"#;

fn bench_small_package(c: &mut Criterion) {
    c.bench_function("sort_small_package", |b| {
        b.iter(|| {
            sort_package_json(SMALL_PACKAGE).unwrap();
        });
    });
}

fn bench_medium_package(c: &mut Criterion) {
    c.bench_function("sort_medium_package", |b| {
        b.iter(|| {
            sort_package_json(MEDIUM_PACKAGE).unwrap();
        });
    });
}

fn bench_large_package(c: &mut Criterion) {
    c.bench_function("sort_large_package", |b| {
        b.iter(|| {
            sort_package_json(LARGE_PACKAGE).unwrap();
        });
    });
}

fn bench_idempotent_sorting(c: &mut Criterion) {
    // Pre-sort the package once
    let sorted = sort_package_json(MEDIUM_PACKAGE).unwrap();
    
    c.bench_function("sort_already_sorted", |b| {
        b.iter(|| {
            sort_package_json(&sorted).unwrap();
        });
    });
}

fn bench_complex_exports(c: &mut Criterion) {
    const COMPLEX_EXPORTS: &str = r#"{
  "name": "exports-test",
  "version": "1.0.0",
  "exports": {
    "./z-path": "./z.js",
    "./a-path": "./a.js",
    ".": {
      "default": "./index.js",
      "node": "./node.js",
      "types": "./index.d.ts",
      "browser": "./browser.js",
      "import": "./esm.js",
      "require": "./cjs.js"
    },
    "./nested": {
      "default": "./nested/index.js",
      "types": "./nested/index.d.ts"
    }
  }
}"#;

    c.bench_function("sort_complex_exports", |b| {
        b.iter(|| {
            sort_package_json(COMPLEX_EXPORTS).unwrap();
        });
    });
}

criterion_group!(
    benches,
    bench_small_package,
    bench_medium_package,
    bench_large_package,
    bench_idempotent_sorting,
    bench_complex_exports
);
criterion_main!(benches);
