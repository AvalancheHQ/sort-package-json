use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sort_package_json::sort_package_json;

// Small package.json with basic fields
const SMALL_PACKAGE: &str = r#"{
  "version": "1.0.0",
  "name": "small-package",
  "description": "A small package",
  "dependencies": {
    "react": "^18.0.0"
  }
}"#;

// Medium package.json with more complex structure
const MEDIUM_PACKAGE: &str = r#"{
  "version": "1.0.0",
  "scripts": {
    "test": "jest",
    "build": "webpack",
    "dev": "webpack serve"
  },
  "name": "medium-package",
  "dependencies": {
    "react": "^18.0.0",
    "lodash": "^4.17.21",
    "axios": "^1.0.0"
  },
  "devDependencies": {
    "webpack": "^5.0.0",
    "jest": "^29.0.0"
  },
  "exports": {
    ".": "./dist/index.js",
    "./utils": "./dist/utils.js"
  }
}"#;

// Large package.json with comprehensive fields
const LARGE_PACKAGE: &str = r#"{
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

fn bench_sort_small_package(c: &mut Criterion) {
    c.bench_function("sort_small_package", |b| {
        b.iter(|| sort_package_json(black_box(SMALL_PACKAGE)))
    });
}

fn bench_sort_medium_package(c: &mut Criterion) {
    c.bench_function("sort_medium_package", |b| {
        b.iter(|| sort_package_json(black_box(MEDIUM_PACKAGE)))
    });
}

fn bench_sort_large_package(c: &mut Criterion) {
    c.bench_function("sort_large_package", |b| {
        b.iter(|| sort_package_json(black_box(LARGE_PACKAGE)))
    });
}

fn bench_sort_idempotent(c: &mut Criterion) {
    // Test the performance of sorting an already-sorted package.json
    let sorted = sort_package_json(LARGE_PACKAGE).unwrap();
    c.bench_function("sort_idempotent", |b| {
        b.iter(|| sort_package_json(black_box(&sorted)))
    });
}

fn bench_sort_with_many_dependencies(c: &mut Criterion) {
    // Create a package.json with many dependencies
    let mut deps = Vec::new();
    for i in 0..50 {
        deps.push(format!("\"package-{}\": \"^1.0.0\"", i));
    }
    let package_with_many_deps = format!(
        r#"{{
  "name": "many-deps",
  "version": "1.0.0",
  "dependencies": {{
    {}
  }}
}}"#,
        deps.join(",\n    ")
    );

    c.bench_function("sort_many_dependencies", |b| {
        b.iter(|| sort_package_json(black_box(&package_with_many_deps)))
    });
}

fn bench_sort_with_complex_exports(c: &mut Criterion) {
    // Create a package.json with complex exports field
    let package_with_exports = r#"{
  "name": "complex-exports",
  "version": "1.0.0",
  "exports": {
    ".": {
      "types": "./index.d.ts",
      "import": "./index.mjs",
      "require": "./index.cjs",
      "default": "./index.js"
    },
    "./feature-a": {
      "types": "./feature-a.d.ts",
      "import": "./feature-a.mjs",
      "default": "./feature-a.js"
    },
    "./feature-b": {
      "types": "./feature-b.d.ts",
      "default": "./feature-b.js"
    },
    "./utils/*": "./utils/*.js",
    "./package.json": "./package.json"
  }
}"#;

    c.bench_function("sort_complex_exports", |b| {
        b.iter(|| sort_package_json(black_box(package_with_exports)))
    });
}

criterion_group!(
    benches,
    bench_sort_small_package,
    bench_sort_medium_package,
    bench_sort_large_package,
    bench_sort_idempotent,
    bench_sort_with_many_dependencies,
    bench_sort_with_complex_exports
);
criterion_main!(benches);
