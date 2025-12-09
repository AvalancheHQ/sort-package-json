use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;
use std::fs;

fn bench_sort_small_package_json(c: &mut Criterion) {
    let input = fs::read_to_string("tests/fixtures/package.json")
        .expect("Failed to read fixture");
    
    c.bench_function("sort_small_package_json", |b| {
        b.iter(|| sort_package_json(&input))
    });
}

fn bench_sort_complex_package_json(c: &mut Criterion) {
    // Create a more complex package.json with many fields
    let complex_json = r#"{
        "version": "2.5.0",
        "dependencies": {
            "zlib": "^1.0.0",
            "axios": "^1.0.0",
            "react": "^18.0.0",
            "lodash": "^4.17.21",
            "express": "^4.18.0",
            "typescript": "^5.0.0",
            "webpack": "^5.0.0",
            "babel-core": "^7.0.0"
        },
        "name": "complex-package",
        "scripts": {
            "test": "jest",
            "build": "webpack",
            "start": "node server.js",
            "dev": "webpack serve",
            "lint": "eslint .",
            "format": "prettier --write .",
            "pretest": "echo 'Starting tests'",
            "posttest": "echo 'Tests complete'",
            "prebuild": "npm run clean",
            "postbuild": "npm run copy-files"
        },
        "description": "A complex test package",
        "keywords": ["test", "complex", "sorting", "json", "example", "test", "demo"],
        "devDependencies": {
            "webpack": "^5.0.0",
            "jest": "^29.0.0",
            "eslint": "^8.0.0",
            "prettier": "^3.0.0",
            "typescript": "^5.0.0",
            "@types/node": "^20.0.0",
            "@types/react": "^18.0.0"
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
            "./helpers": {
                "import": "./dist/helpers.esm.js",
                "types": "./dist/helpers.d.ts",
                "default": "./dist/helpers.js"
            },
            "./package.json": "./package.json"
        },
        "engines": {
            "node": ">=18.0.0",
            "npm": ">=8.0.0"
        },
        "author": {
            "url": "https://example.com",
            "email": "author@example.com",
            "name": "Test Author"
        },
        "maintainers": [
            {"name": "Maintainer One", "email": "m1@example.com", "url": "https://m1.com"},
            {"name": "Maintainer Two", "email": "m2@example.com", "url": "https://m2.com"}
        ],
        "babel": {
            "presets": ["@babel/preset-env", "@babel/preset-react"],
            "plugins": ["@babel/plugin-proposal-class-properties"]
        },
        "jest": {
            "testEnvironment": "node",
            "coverageDirectory": "coverage",
            "collectCoverageFrom": ["src/**/*.js"]
        },
        "customField1": "custom value 1",
        "customField2": "custom value 2",
        "_private1": "private field 1",
        "_private2": "private field 2"
    }"#;
    
    c.bench_function("sort_complex_package_json", |b| {
        b.iter(|| sort_package_json(complex_json))
    });
}

fn bench_sort_minimal_package_json(c: &mut Criterion) {
    let minimal_json = r#"{
        "version": "1.0.0",
        "name": "minimal-package",
        "description": "Minimal package.json"
    }"#;
    
    c.bench_function("sort_minimal_package_json", |b| {
        b.iter(|| sort_package_json(minimal_json))
    });
}

fn bench_sort_idempotent(c: &mut Criterion) {
    let input = fs::read_to_string("tests/fixtures/package.json")
        .expect("Failed to read fixture");
    let sorted_once = sort_package_json(&input).expect("Failed to sort");
    
    c.bench_function("sort_already_sorted_package_json", |b| {
        b.iter(|| sort_package_json(&sorted_once))
    });
}

criterion_group!(
    benches,
    bench_sort_small_package_json,
    bench_sort_complex_package_json,
    bench_sort_minimal_package_json,
    bench_sort_idempotent
);
criterion_main!(benches);
