use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

const SIMPLE_PACKAGE_JSON: &str = r#"{
  "name": "test-package",
  "version": "1.0.0",
  "description": "A test package",
  "main": "./dist/index.js",
  "scripts": {
    "test": "jest",
    "build": "webpack"
  },
  "dependencies": {
    "react": "^18.0.0"
  }
}"#;

const COMPLEX_PACKAGE_JSON: &str = r#"{
  "name": "test-package",
  "version": "1.0.0",
  "description": "A test package for sorting",
  "keywords": [
    "test",
    "sorting",
    "json",
    "test",
    "example"
  ],
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
  "maintainers": [
    {
      "url": "https://maintainer1.com",
      "name": "Maintainer One",
      "email": "maintainer1@example.com"
    },
    {
      "email": "maintainer2@example.com",
      "url": "https://maintainer2.com",
      "name": "Maintainer Two"
    }
  ],
  "exports": {
    ".": {
      "default": "./dist/index.js",
      "require": "./dist/index.cjs",
      "types": "./dist/index.d.ts",
      "import": "./dist/index.esm.js"
    },
    "./package.json": "./package.json",
    "./utils": {
      "import": "./dist/utils.esm.js",
      "default": "./dist/utils.js",
      "types": "./dist/utils.d.ts"
    }
  },
  "main": "./dist/index.js",
  "module": "./dist/index.esm.js",
  "types": "./dist/index.d.ts",
  "files": [
    "src",
    "dist",
    "README.md",
    "dist",
    "LICENSE"
  ],
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
  },
  "babel": {
    "presets": ["@babel/preset-env", "@babel/preset-react"],
    "plugins": ["@babel/plugin-proposal-class-properties"]
  },
  "publishConfig": {
    "access": "public"
  },
  "customField": "this is a custom unknown field",
  "_custom": "another private field",
  "_id": "test-package@1.0.0"
}"#;

fn bench_simple_package(c: &mut Criterion) {
    c.bench_function("sort_simple_package_json", |b| {
        b.iter(|| sort_package_json(black_box(SIMPLE_PACKAGE_JSON)))
    });
}

fn bench_complex_package(c: &mut Criterion) {
    c.bench_function("sort_complex_package_json", |b| {
        b.iter(|| sort_package_json(black_box(COMPLEX_PACKAGE_JSON)))
    });
}

fn bench_large_dependencies(c: &mut Criterion) {
    let mut large_package = String::from(r#"{"name":"test","version":"1.0.0","dependencies":{"#);
    for i in 0..100 {
        if i > 0 {
            large_package.push(',');
        }
        large_package.push_str(&format!(r#""package-{}":"^{}.0.0""#, i, i % 10));
    }
    large_package.push_str("}}");

    c.bench_function("sort_large_dependencies", |b| {
        b.iter(|| sort_package_json(black_box(&large_package)))
    });
}

fn bench_many_scripts(c: &mut Criterion) {
    let mut scripts_package = String::from(r#"{"name":"test","version":"1.0.0","scripts":{"#);
    for i in 0..50 {
        if i > 0 {
            scripts_package.push(',');
        }
        scripts_package.push_str(&format!(r#""script-{}":"echo {}""#, i, i));
    }
    scripts_package.push_str("}}");

    c.bench_function("sort_many_scripts", |b| {
        b.iter(|| sort_package_json(black_box(&scripts_package)))
    });
}

criterion_group!(
    benches,
    bench_simple_package,
    bench_complex_package,
    bench_large_dependencies,
    bench_many_scripts
);
criterion_main!(benches);
