use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

const SIMPLE_PACKAGE_JSON: &str = r#"{
  "version": "1.0.0",
  "name": "test-package",
  "dependencies": {
    "z-package": "1.0.0",
    "a-package": "1.0.0",
    "m-package": "1.0.0"
  },
  "devDependencies": {
    "test-package": "1.0.0",
    "another-package": "1.0.0"
  },
  "scripts": {
    "test": "jest",
    "build": "tsc",
    "start": "node index.js"
  },
  "keywords": ["sorting", "package", "json", "test"],
  "author": "Test Author",
  "license": "MIT"
}"#;

const COMPLEX_PACKAGE_JSON: &str = r#"{
  "scripts": {
    "pretest": "npm run lint",
    "test": "jest",
    "posttest": "npm run coverage",
    "prebuild": "npm run clean",
    "build": "tsc",
    "postbuild": "npm run docs",
    "start": "node index.js",
    "dev": "nodemon index.js",
    "lint": "eslint .",
    "format": "prettier --write .",
    "clean": "rm -rf dist"
  },
  "version": "2.5.3",
  "name": "complex-package",
  "description": "A complex package with many fields",
  "keywords": ["test", "benchmark", "sorting", "package", "json", "complex"],
  "dependencies": {
    "express": "^4.18.0",
    "lodash": "^4.17.21",
    "react": "^18.2.0",
    "axios": "^1.4.0"
  },
  "devDependencies": {
    "typescript": "^5.0.0",
    "jest": "^29.0.0",
    "eslint": "^8.0.0",
    "prettier": "^3.0.0",
    "@types/node": "^20.0.0",
    "@types/express": "^4.17.0"
  },
  "peerDependencies": {
    "react": ">=16.8.0"
  },
  "engines": {
    "node": ">=18.0.0",
    "npm": ">=9.0.0"
  },
  "repository": {
    "type": "git",
    "url": "https://github.com/example/complex-package"
  },
  "bugs": {
    "url": "https://github.com/example/complex-package/issues",
    "email": "bugs@example.com"
  },
  "author": {
    "name": "John Doe",
    "email": "john@example.com",
    "url": "https://example.com"
  },
  "contributors": [
    {
      "name": "Jane Smith",
      "email": "jane@example.com"
    },
    {
      "name": "Bob Johnson",
      "email": "bob@example.com"
    }
  ],
  "license": "MIT",
  "private": false,
  "files": ["dist", "README.md", "LICENSE"],
  "main": "dist/index.js",
  "types": "dist/index.d.ts",
  "exports": {
    ".": {
      "types": "./dist/index.d.ts",
      "require": "./dist/index.js",
      "import": "./dist/index.mjs"
    },
    "./utils": {
      "types": "./dist/utils.d.ts",
      "require": "./dist/utils.js",
      "import": "./dist/utils.mjs"
    }
  }
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

fn bench_large_dependencies(c: &mut Criterion) {
    let mut package_json = r#"{
  "name": "large-deps-package",
  "version": "1.0.0",
  "dependencies": {
"#
    .to_string();

    // Generate a package.json with 50 dependencies
    for i in 0..50 {
        package_json.push_str(&format!("    \"package-{}\": \"1.0.0\"", i));
        if i < 49 {
            package_json.push_str(",\n");
        } else {
            package_json.push('\n');
        }
    }

    package_json.push_str("  }\n}");

    c.bench_function("sort_large_dependencies", |b| {
        b.iter(|| sort_package_json(&package_json))
    });
}

fn bench_nested_objects(c: &mut Criterion) {
    let nested_package_json = r#"{
  "name": "nested-package",
  "version": "1.0.0",
  "jest": {
    "testEnvironment": "node",
    "coverageDirectory": "coverage",
    "collectCoverageFrom": ["src/**/*.js"],
    "testMatch": ["**/__tests__/**/*.js"],
    "moduleNameMapper": {
      "^@/(.*)$": "<rootDir>/src/$1"
    }
  },
  "babel": {
    "presets": [
      ["@babel/preset-env", {
        "targets": {
          "node": "current"
        }
      }]
    ],
    "plugins": [
      "@babel/plugin-proposal-class-properties",
      "@babel/plugin-transform-runtime"
    ]
  },
  "eslintConfig": {
    "env": {
      "node": true,
      "jest": true
    },
    "extends": ["eslint:recommended"],
    "rules": {
      "no-console": "warn",
      "no-unused-vars": "error"
    }
  }
}"#;

    c.bench_function("sort_nested_objects", |b| {
        b.iter(|| sort_package_json(nested_package_json))
    });
}

criterion_group!(
    benches,
    bench_simple_package_json,
    bench_complex_package_json,
    bench_large_dependencies,
    bench_nested_objects
);
criterion_main!(benches);
