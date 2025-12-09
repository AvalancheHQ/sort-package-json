use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;
use std::fs;

fn bench_sort_small_package_json(c: &mut Criterion) {
    let input =
        fs::read_to_string("tests/fixtures/package.json").expect("Failed to read fixture file");
    c.bench_function("sort_small_package_json", |b| {
        b.iter(|| sort_package_json(&input).expect("Failed to sort package.json"));
    });
}

fn bench_sort_minimal_package_json(c: &mut Criterion) {
    let input = r#"{
        "version": "1.0.0",
        "name": "my-package",
        "description": "A simple package"
    }"#;
    c.bench_function("sort_minimal_package_json", |b| {
        b.iter(|| sort_package_json(input).expect("Failed to sort package.json"));
    });
}

fn bench_sort_large_package_json(c: &mut Criterion) {
    // Simulate a large package.json with many dependencies
    let input = r#"{
        "version": "1.0.0",
        "name": "large-package",
        "description": "A large package with many dependencies",
        "scripts": {
            "test": "jest",
            "build": "webpack",
            "dev": "webpack serve",
            "lint": "eslint .",
            "format": "prettier --write .",
            "typecheck": "tsc --noEmit",
            "prepare": "husky install",
            "pretest": "echo 'Starting tests'",
            "posttest": "echo 'Tests complete'",
            "prebuild": "npm run lint",
            "postbuild": "npm run typecheck"
        },
        "dependencies": {
            "react": "^18.0.0",
            "react-dom": "^18.0.0",
            "axios": "^1.0.0",
            "lodash": "^4.17.21",
            "express": "^4.18.0",
            "mongoose": "^7.0.0",
            "jsonwebtoken": "^9.0.0",
            "bcrypt": "^5.1.0",
            "cors": "^2.8.5",
            "dotenv": "^16.0.0",
            "helmet": "^7.0.0",
            "morgan": "^1.10.0",
            "winston": "^3.8.0",
            "joi": "^17.9.0",
            "moment": "^2.29.4",
            "uuid": "^9.0.0",
            "dayjs": "^1.11.0",
            "sharp": "^0.32.0",
            "multer": "^1.4.5",
            "socket.io": "^4.6.0",
            "redis": "^4.6.0",
            "bull": "^4.11.0",
            "nodemailer": "^6.9.0",
            "stripe": "^12.0.0",
            "passport": "^0.6.0",
            "passport-jwt": "^4.0.0",
            "passport-google-oauth20": "^2.0.0",
            "passport-facebook": "^3.0.0"
        },
        "devDependencies": {
            "webpack": "^5.0.0",
            "jest": "^29.0.0",
            "typescript": "^5.0.0",
            "@types/node": "^20.0.0",
            "@types/react": "^18.0.0",
            "@types/react-dom": "^18.0.0",
            "@types/express": "^4.17.0",
            "@types/cors": "^2.8.0",
            "@types/bcrypt": "^5.0.0",
            "@types/jsonwebtoken": "^9.0.0",
            "@types/morgan": "^1.9.0",
            "@types/multer": "^1.4.0",
            "@types/passport": "^1.0.0",
            "@types/passport-jwt": "^3.0.0",
            "eslint": "^8.40.0",
            "prettier": "^2.8.0",
            "husky": "^8.0.0",
            "lint-staged": "^13.2.0",
            "@babel/core": "^7.22.0",
            "@babel/preset-env": "^7.22.0",
            "@babel/preset-react": "^7.22.0",
            "@babel/preset-typescript": "^7.22.0",
            "webpack-cli": "^5.1.0",
            "webpack-dev-server": "^4.15.0",
            "ts-node": "^10.9.0",
            "ts-jest": "^29.1.0",
            "nodemon": "^2.0.22",
            "@testing-library/react": "^14.0.0",
            "@testing-library/jest-dom": "^5.16.5"
        },
        "keywords": [
            "react",
            "express",
            "mongodb",
            "typescript",
            "node",
            "api",
            "rest",
            "backend",
            "frontend",
            "fullstack",
            "web",
            "app"
        ],
        "engines": {
            "node": ">=18.0.0",
            "npm": ">=8.0.0"
        }
    }"#;

    c.bench_function("sort_large_package_json", |b| {
        b.iter(|| sort_package_json(input).expect("Failed to sort package.json"));
    });
}

fn bench_sort_complex_exports(c: &mut Criterion) {
    let input = r#"{
        "name": "complex-exports-package",
        "version": "1.0.0",
        "exports": {
            "./utils": {
                "import": "./dist/utils.esm.js",
                "default": "./dist/utils.js",
                "require": "./dist/utils.cjs",
                "types": "./dist/utils.d.ts"
            },
            ".": {
                "types": "./dist/index.d.ts",
                "default": "./dist/index.js",
                "import": "./dist/index.esm.js",
                "require": "./dist/index.cjs"
            },
            "./package.json": "./package.json",
            "./components/*": {
                "import": "./dist/components/*.esm.js",
                "types": "./dist/components/*.d.ts",
                "default": "./dist/components/*.js"
            }
        }
    }"#;

    c.bench_function("sort_complex_exports", |b| {
        b.iter(|| sort_package_json(input).expect("Failed to sort package.json"));
    });
}

fn bench_sort_with_nested_objects(c: &mut Criterion) {
    let input = r#"{
        "name": "nested-config-package",
        "version": "1.0.0",
        "babel": {
            "presets": ["@babel/preset-env", "@babel/preset-react"],
            "plugins": ["@babel/plugin-proposal-class-properties"],
            "env": {
                "test": {
                    "plugins": ["@babel/plugin-transform-runtime"]
                }
            }
        },
        "jest": {
            "testEnvironment": "node",
            "coverageDirectory": "coverage",
            "collectCoverageFrom": ["src/**/*.{js,jsx,ts,tsx}"],
            "testMatch": ["**/__tests__/**/*.[jt]s?(x)", "**/?(*.)+(spec|test).[jt]s?(x)"]
        }
    }"#;

    c.bench_function("sort_with_nested_objects", |b| {
        b.iter(|| sort_package_json(input).expect("Failed to sort package.json"));
    });
}

fn bench_idempotent_sort(c: &mut Criterion) {
    let input =
        fs::read_to_string("tests/fixtures/package.json").expect("Failed to read fixture file");
    let sorted_once = sort_package_json(&input).expect("Failed to sort package.json");

    c.bench_function("sort_idempotent", |b| {
        b.iter(|| sort_package_json(&sorted_once).expect("Failed to sort package.json"));
    });
}

criterion_group!(
    benches,
    bench_sort_small_package_json,
    bench_sort_minimal_package_json,
    bench_sort_large_package_json,
    bench_sort_complex_exports,
    bench_sort_with_nested_objects,
    bench_idempotent_sort
);
criterion_main!(benches);
