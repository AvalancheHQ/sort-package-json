use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

const SIMPLE_PACKAGE_JSON: &str = r#"{
    "name": "example",
    "version": "1.0.0",
    "description": "A test package",
    "main": "index.js",
    "scripts": {
        "test": "jest",
        "build": "webpack"
    },
    "dependencies": {
        "react": "^18.0.0",
        "axios": "^1.0.0"
    },
    "devDependencies": {
        "webpack": "^5.0.0",
        "jest": "^29.0.0"
    },
    "keywords": ["test", "example"],
    "author": "Test Author",
    "license": "MIT"
}"#;

const COMPLEX_PACKAGE_JSON: &str = r#"{
    "scripts": {
        "test": "jest",
        "build": "webpack",
        "lint": "eslint .",
        "format": "prettier --write .",
        "dev": "webpack serve"
    },
    "name": "complex-example",
    "dependencies": {
        "react": "^18.0.0",
        "react-dom": "^18.0.0",
        "axios": "^1.0.0",
        "lodash": "^4.17.21",
        "redux": "^4.2.0"
    },
    "devDependencies": {
        "webpack": "^5.0.0",
        "webpack-cli": "^5.0.0",
        "jest": "^29.0.0",
        "eslint": "^8.0.0",
        "prettier": "^2.8.0"
    },
    "version": "2.5.1",
    "description": "A complex package with many dependencies",
    "main": "dist/index.js",
    "keywords": ["react", "webpack", "testing", "example"],
    "author": {
        "name": "Test Author",
        "email": "test@example.com",
        "url": "https://example.com"
    },
    "license": "MIT",
    "repository": {
        "type": "git",
        "url": "https://github.com/example/repo.git"
    },
    "bugs": {
        "url": "https://github.com/example/repo/issues"
    },
    "homepage": "https://example.com",
    "engines": {
        "node": ">=16.0.0",
        "npm": ">=8.0.0"
    }
}"#;

const EXPORTS_PACKAGE_JSON: &str = r#"{
    "name": "exports-example",
    "version": "1.0.0",
    "exports": {
        "./package.json": "./package.json",
        ".": {
            "types": "./dist/index.d.ts",
            "import": "./dist/index.mjs",
            "require": "./dist/index.cjs",
            "default": "./dist/index.js"
        },
        "./utils": {
            "types": "./dist/utils.d.ts",
            "import": "./dist/utils.mjs",
            "require": "./dist/utils.cjs"
        }
    }
}"#;

fn bench_simple_package(c: &mut Criterion) {
    c.bench_function("sort_simple_package_json", |b| {
        b.iter(|| sort_package_json(SIMPLE_PACKAGE_JSON))
    });
}

fn bench_complex_package(c: &mut Criterion) {
    c.bench_function("sort_complex_package_json", |b| {
        b.iter(|| sort_package_json(COMPLEX_PACKAGE_JSON))
    });
}

fn bench_exports_package(c: &mut Criterion) {
    c.bench_function("sort_exports_package_json", |b| {
        b.iter(|| sort_package_json(EXPORTS_PACKAGE_JSON))
    });
}

fn bench_nested_objects(c: &mut Criterion) {
    let nested_json = r#"{
        "name": "nested-example",
        "version": "1.0.0",
        "jest": {
            "testEnvironment": "node",
            "coverageDirectory": "coverage",
            "collectCoverageFrom": ["src/**/*.js"],
            "testMatch": ["**/*.test.js"]
        },
        "babel": {
            "presets": ["@babel/preset-env", "@babel/preset-react"],
            "plugins": ["@babel/plugin-proposal-class-properties"]
        }
    }"#;
    
    c.bench_function("sort_nested_objects", |b| {
        b.iter(|| sort_package_json(nested_json))
    });
}

criterion_group!(
    benches,
    bench_simple_package,
    bench_complex_package,
    bench_exports_package,
    bench_nested_objects
);
criterion_main!(benches);
