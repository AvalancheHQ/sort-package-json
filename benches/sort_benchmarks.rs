use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

fn bench_simple_package_json(c: &mut Criterion) {
    let input = r#"{
        "version": "1.0.0",
        "name": "my-package",
        "dependencies": {
            "lodash": "^4.17.21",
            "express": "^4.18.2"
        },
        "scripts": {
            "test": "jest",
            "build": "webpack"
        }
    }"#;

    c.bench_function("simple_package_json", |b| {
        b.iter(|| sort_package_json(input))
    });
}

fn bench_complex_package_json(c: &mut Criterion) {
    let input = r#"{
        "version": "1.0.0",
        "name": "complex-package",
        "description": "A complex package",
        "keywords": ["test", "benchmark", "sorting"],
        "homepage": "https://example.com",
        "bugs": {
            "url": "https://github.com/example/repo/issues"
        },
        "license": "MIT",
        "author": {
            "name": "John Doe",
            "email": "john@example.com"
        },
        "files": ["dist", "lib", "src"],
        "main": "dist/index.js",
        "module": "dist/index.esm.js",
        "types": "dist/index.d.ts",
        "scripts": {
            "test": "jest",
            "test:watch": "jest --watch",
            "build": "webpack",
            "lint": "eslint .",
            "format": "prettier --write ."
        },
        "dependencies": {
            "react": "^18.2.0",
            "lodash": "^4.17.21",
            "axios": "^1.3.0"
        },
        "devDependencies": {
            "webpack": "^5.75.0",
            "jest": "^29.4.0",
            "eslint": "^8.34.0",
            "prettier": "^2.8.4"
        },
        "peerDependencies": {
            "react": ">=16.8.0"
        },
        "engines": {
            "node": ">=14.0.0",
            "npm": ">=6.0.0"
        }
    }"#;

    c.bench_function("complex_package_json", |b| {
        b.iter(|| sort_package_json(input))
    });
}

fn bench_large_dependencies(c: &mut Criterion) {
    let input = r#"{
        "name": "large-deps",
        "version": "1.0.0",
        "dependencies": {
            "react": "^18.2.0",
            "react-dom": "^18.2.0",
            "lodash": "^4.17.21",
            "axios": "^1.3.0",
            "express": "^4.18.2",
            "moment": "^2.29.4",
            "typescript": "^4.9.5",
            "webpack": "^5.75.0",
            "babel-loader": "^9.1.2",
            "eslint": "^8.34.0",
            "prettier": "^2.8.4",
            "jest": "^29.4.0",
            "dotenv": "^16.0.3",
            "body-parser": "^1.20.1",
            "cors": "^2.8.5",
            "jsonwebtoken": "^9.0.0",
            "bcrypt": "^5.1.0",
            "mongoose": "^6.9.1",
            "redis": "^4.6.4",
            "pg": "^8.9.0"
        }
    }"#;

    c.bench_function("large_dependencies", |b| {
        b.iter(|| sort_package_json(input))
    });
}

fn bench_exports_field(c: &mut Criterion) {
    let input = r#"{
        "name": "exports-package",
        "version": "1.0.0",
        "exports": {
            ".": {
                "import": "./dist/index.mjs",
                "require": "./dist/index.js",
                "types": "./dist/index.d.ts"
            },
            "./utils": {
                "import": "./dist/utils.mjs",
                "require": "./dist/utils.js"
            },
            "./package.json": "./package.json"
        }
    }"#;

    c.bench_function("exports_field", |b| {
        b.iter(|| sort_package_json(input))
    });
}

fn bench_scripts_sorting(c: &mut Criterion) {
    let input = r#"{
        "name": "scripts-package",
        "version": "1.0.0",
        "scripts": {
            "test": "jest",
            "test:watch": "jest --watch",
            "test:coverage": "jest --coverage",
            "build": "webpack",
            "build:dev": "webpack --mode development",
            "build:prod": "webpack --mode production",
            "lint": "eslint .",
            "lint:fix": "eslint . --fix",
            "format": "prettier --write .",
            "format:check": "prettier --check .",
            "start": "node server.js",
            "dev": "nodemon server.js",
            "clean": "rm -rf dist",
            "prepare": "husky install",
            "prepublishOnly": "npm test && npm run build"
        }
    }"#;

    c.bench_function("scripts_sorting", |b| {
        b.iter(|| sort_package_json(input))
    });
}

fn bench_nested_config_objects(c: &mut Criterion) {
    let input = r#"{
        "name": "config-package",
        "version": "1.0.0",
        "jest": {
            "testEnvironment": "node",
            "coverageDirectory": "./coverage",
            "collectCoverageFrom": ["src/**/*.js"],
            "testMatch": ["**/__tests__/**/*.js", "**/?(*.)+(spec|test).js"]
        },
        "babel": {
            "presets": ["@babel/preset-env", "@babel/preset-react"],
            "plugins": ["@babel/plugin-proposal-class-properties"]
        },
        "eslintConfig": {
            "extends": ["eslint:recommended", "plugin:react/recommended"],
            "env": {
                "browser": true,
                "node": true,
                "es2021": true
            },
            "rules": {
                "indent": ["error", 2],
                "quotes": ["error", "single"]
            }
        }
    }"#;

    c.bench_function("nested_config_objects", |b| {
        b.iter(|| sort_package_json(input))
    });
}

fn bench_minimal_package_json(c: &mut Criterion) {
    let input = r#"{
        "name": "minimal",
        "version": "1.0.0"
    }"#;

    c.bench_function("minimal_package_json", |b| {
        b.iter(|| sort_package_json(input))
    });
}

fn bench_monorepo_workspaces(c: &mut Criterion) {
    let input = r#"{
        "name": "monorepo",
        "version": "1.0.0",
        "private": true,
        "workspaces": [
            "packages/*",
            "apps/*"
        ],
        "scripts": {
            "build": "lerna run build",
            "test": "lerna run test",
            "clean": "lerna clean"
        },
        "devDependencies": {
            "lerna": "^6.5.0",
            "typescript": "^4.9.5"
        }
    }"#;

    c.bench_function("monorepo_workspaces", |b| {
        b.iter(|| sort_package_json(input))
    });
}

criterion_group!(
    benches,
    bench_simple_package_json,
    bench_complex_package_json,
    bench_large_dependencies,
    bench_exports_field,
    bench_scripts_sorting,
    bench_nested_config_objects,
    bench_minimal_package_json,
    bench_monorepo_workspaces
);
criterion_main!(benches);
