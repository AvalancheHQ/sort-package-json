use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;
use std::fs;

fn benchmark_sort_package_json(c: &mut Criterion) {
    // Load the test fixture
    let input = fs::read_to_string("tests/fixtures/package.json")
        .expect("Failed to read fixture");

    c.bench_function("sort_package_json", |b| {
        b.iter(|| {
            sort_package_json(black_box(&input))
                .expect("Failed to parse package.json")
        });
    });
}

fn benchmark_sort_idempotency(c: &mut Criterion) {
    // Load and pre-sort the fixture
    let input = fs::read_to_string("tests/fixtures/package.json")
        .expect("Failed to read fixture");
    let sorted = sort_package_json(&input)
        .expect("Failed to parse package.json");

    c.bench_function("sort_package_json_idempotent", |b| {
        b.iter(|| {
            sort_package_json(black_box(&sorted))
                .expect("Failed to parse package.json")
        });
    });
}

fn benchmark_sort_minimal(c: &mut Criterion) {
    let minimal = r#"{
        "version": "1.0.0",
        "name": "test",
        "dependencies": {}
    }"#;

    c.bench_function("sort_minimal_package_json", |b| {
        b.iter(|| {
            sort_package_json(black_box(minimal))
                .expect("Failed to parse package.json")
        });
    });
}

fn benchmark_sort_large(c: &mut Criterion) {
    // Create a larger package.json with many dependencies
    let large = r#"{
        "name": "large-package",
        "version": "1.0.0",
        "description": "A package with many dependencies",
        "keywords": ["test", "benchmark", "performance", "json", "sorting"],
        "homepage": "https://github.com/test/large-package",
        "repository": {
            "type": "git",
            "url": "https://github.com/test/large-package"
        },
        "license": "MIT",
        "main": "./dist/index.js",
        "module": "./dist/index.esm.js",
        "types": "./dist/index.d.ts",
        "scripts": {
            "build": "webpack",
            "test": "jest",
            "lint": "eslint .",
            "format": "prettier --write .",
            "dev": "webpack serve",
            "prebuild": "npm run clean",
            "postbuild": "npm run test",
            "clean": "rm -rf dist"
        },
        "dependencies": {
            "axios": "^1.0.0",
            "lodash": "^4.17.21",
            "react": "^18.0.0",
            "react-dom": "^18.0.0",
            "express": "^4.18.0",
            "moment": "^2.29.4",
            "uuid": "^9.0.0",
            "chalk": "^5.0.0",
            "dotenv": "^16.0.0",
            "cors": "^2.8.5"
        },
        "devDependencies": {
            "webpack": "^5.0.0",
            "jest": "^29.0.0",
            "typescript": "^5.0.0",
            "eslint": "^8.0.0",
            "prettier": "^3.0.0",
            "@types/react": "^18.0.0",
            "@types/node": "^20.0.0",
            "webpack-cli": "^5.0.0",
            "webpack-dev-server": "^4.0.0",
            "ts-loader": "^9.0.0"
        },
        "engines": {
            "node": ">=18.0.0",
            "npm": ">=8.0.0"
        }
    }"#;

    c.bench_function("sort_large_package_json", |b| {
        b.iter(|| {
            sort_package_json(black_box(large))
                .expect("Failed to parse package.json")
        });
    });
}

criterion_group!(
    benches,
    benchmark_sort_package_json,
    benchmark_sort_idempotency,
    benchmark_sort_minimal,
    benchmark_sort_large
);
criterion_main!(benches);
