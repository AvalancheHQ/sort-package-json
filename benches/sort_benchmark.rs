use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;
use std::fs;

fn bench_sort_small_package_json(c: &mut Criterion) {
    let input = fs::read_to_string("tests/fixtures/package.json")
        .expect("Failed to read fixture");
    
    c.bench_function("sort_small_package_json", |b| {
        b.iter(|| {
            sort_package_json(&input).expect("Failed to sort")
        })
    });
}

fn bench_sort_already_sorted(c: &mut Criterion) {
    let input = fs::read_to_string("tests/fixtures/package.json")
        .expect("Failed to read fixture");
    let sorted = sort_package_json(&input).expect("Failed to sort");
    
    c.bench_function("sort_already_sorted", |b| {
        b.iter(|| {
            sort_package_json(&sorted).expect("Failed to sort")
        })
    });
}

fn bench_sort_minimal_package_json(c: &mut Criterion) {
    let minimal = r#"{
        "name": "minimal-package",
        "version": "1.0.0",
        "description": "Minimal package"
    }"#;
    
    c.bench_function("sort_minimal_package_json", |b| {
        b.iter(|| {
            sort_package_json(minimal).expect("Failed to sort")
        })
    });
}

fn bench_sort_complex_package_json(c: &mut Criterion) {
    let complex = r#"{
        "name": "complex-package",
        "version": "1.0.0",
        "scripts": {
            "test": "jest",
            "build": "webpack",
            "dev": "webpack serve",
            "lint": "eslint .",
            "format": "prettier --write .",
            "pretest": "echo 'Starting'",
            "posttest": "echo 'Done'"
        },
        "dependencies": {
            "react": "^18.0.0",
            "vue": "^3.0.0",
            "angular": "^14.0.0",
            "svelte": "^3.0.0",
            "axios": "^1.0.0",
            "lodash": "^4.17.21",
            "express": "^4.18.0",
            "next": "^13.0.0"
        },
        "devDependencies": {
            "webpack": "^5.0.0",
            "jest": "^29.0.0",
            "typescript": "^5.0.0",
            "eslint": "^8.0.0",
            "prettier": "^2.8.0"
        },
        "keywords": [
            "test",
            "sorting",
            "json",
            "package",
            "example",
            "benchmark"
        ],
        "exports": {
            ".": {
                "import": "./dist/index.mjs",
                "require": "./dist/index.cjs",
                "types": "./dist/index.d.ts",
                "default": "./dist/index.js"
            },
            "./utils": {
                "import": "./dist/utils.mjs",
                "require": "./dist/utils.cjs",
                "types": "./dist/utils.d.ts"
            },
            "./helpers": {
                "types": "./dist/helpers.d.ts",
                "import": "./dist/helpers.mjs",
                "default": "./dist/helpers.js"
            }
        }
    }"#;
    
    c.bench_function("sort_complex_package_json", |b| {
        b.iter(|| {
            sort_package_json(complex).expect("Failed to sort")
        })
    });
}

criterion_group!(
    benches,
    bench_sort_small_package_json,
    bench_sort_already_sorted,
    bench_sort_minimal_package_json,
    bench_sort_complex_package_json
);
criterion_main!(benches);
