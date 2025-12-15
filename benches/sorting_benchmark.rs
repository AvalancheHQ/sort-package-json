use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use sort_package_json::sort_package_json;
use std::fs;

fn benchmark_sorting(c: &mut Criterion) {
    // Load the fixture file
    let input = fs::read_to_string("tests/fixtures/package.json")
        .expect("Failed to read fixture file");

    c.bench_function("sort_package_json", |b| {
        b.iter(|| sort_package_json(black_box(&input)))
    });
}

fn benchmark_sorting_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("package_json_by_size");

    // Small package.json (minimal fields)
    let small = r#"{
        "name": "small-package",
        "version": "1.0.0",
        "description": "A minimal package",
        "main": "index.js"
    }"#;

    // Medium package.json (typical project)
    let medium = fs::read_to_string("tests/fixtures/package.json")
        .expect("Failed to read fixture file");

    // Large package.json (many dependencies and fields)
    let large = r#"{
        "name": "large-package",
        "version": "1.0.0",
        "description": "A large package with many dependencies",
        "keywords": ["test", "benchmark", "large", "dependencies", "performance"],
        "homepage": "https://github.com/test/large-package",
        "bugs": {"url": "https://github.com/test/large-package/issues"},
        "repository": {"type": "git", "url": "https://github.com/test/large-package"},
        "license": "MIT",
        "author": "Test Author",
        "main": "dist/index.js",
        "module": "dist/index.esm.js",
        "types": "dist/index.d.ts",
        "exports": {
            ".": {
                "types": "./dist/index.d.ts",
                "import": "./dist/index.esm.js",
                "require": "./dist/index.cjs",
                "default": "./dist/index.js"
            },
            "./utils": {"types": "./dist/utils.d.ts", "import": "./dist/utils.esm.js"}
        },
        "files": ["dist", "src", "README.md"],
        "scripts": {
            "build": "tsc && rollup -c",
            "test": "jest",
            "lint": "eslint src",
            "format": "prettier --write src",
            "pretest": "npm run lint",
            "posttest": "npm run coverage"
        },
        "dependencies": {
            "axios": "^1.0.0",
            "lodash": "^4.17.21",
            "react": "^18.0.0",
            "react-dom": "^18.0.0",
            "typescript": "^5.0.0",
            "webpack": "^5.0.0"
        },
        "devDependencies": {
            "eslint": "^8.0.0",
            "jest": "^29.0.0",
            "prettier": "^3.0.0",
            "rollup": "^4.0.0",
            "ts-node": "^10.0.0"
        },
        "peerDependencies": {
            "react": ">=17.0.0"
        },
        "engines": {"node": ">=18.0.0", "npm": ">=8.0.0"}
    }"#;

    group.bench_with_input(BenchmarkId::new("small", "4_fields"), &small, |b, input| {
        b.iter(|| sort_package_json(black_box(input)))
    });

    group.bench_with_input(BenchmarkId::new("medium", "20_fields"), &medium, |b, input| {
        b.iter(|| sort_package_json(black_box(input)))
    });

    group.bench_with_input(BenchmarkId::new("large", "30_fields"), &large, |b, input| {
        b.iter(|| sort_package_json(black_box(input)))
    });

    group.finish();
}

fn benchmark_idempotency(c: &mut Criterion) {
    // Test the performance of sorting an already sorted package.json
    let input = fs::read_to_string("tests/fixtures/package.json")
        .expect("Failed to read fixture file");
    let sorted_once = sort_package_json(&input).expect("Failed to sort");

    c.bench_function("sort_already_sorted", |b| {
        b.iter(|| sort_package_json(black_box(&sorted_once)))
    });
}

criterion_group!(benches, benchmark_sorting, benchmark_sorting_sizes, benchmark_idempotency);
criterion_main!(benches);
