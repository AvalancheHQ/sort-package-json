use serde_json::{Map, Value};

const FIELDS_ORDER: &[&str] = &[
  "$schema",
  "name",
  "displayName",
  "version",
  "stableVersion",
  "private",
  "description",
  "categories",
  "keywords",
  "homepage",
  "bugs",
  "repository",
  "funding",
  "license",
  "qna",
  "author",
  "maintainers",
  "contributors",
  "publisher",
  "sideEffects",
  "type",
  "imports",
  "exports",
  "main",
  "svelte",
  "umd:main",
  "jsdelivr",
  "unpkg",
  "module",
  "source",
  "jsnext:main",
  "browser",
  "react-native",
  "types",
  "typesVersions",
  "typings",
  "style",
  "example",
  "examplestyle",
  "assets",
  "bin",
  "man",
  "directories",
  "files",
  "workspaces",
  "binary",
  "scripts",
  "betterScripts",
  "l10n",
  "contributes",
  "activationEvents",
  "husky",
  "simple-git-hooks",
  "pre-commit",
  "commitlint",
  "lint-staged",
  "nano-staged",
  "config",
  "nodemonConfig",
  "browserify",
  "babel",
  "browserslist",
  "xo",
  "prettier",
  "eslintConfig",
  "eslintIgnore",
  "npmpkgjsonlint",
  "npmPackageJsonLintConfig",
  "npmpackagejsonlint",
  "release",
  "remarkConfig",
  "stylelint",
  "ava",
  "jest",
  "jest-junit",
  "jest-stare",
  "mocha",
  "nyc",
  "c8",
  "tap",
  "oclif",
  "resolutions",
  "overrides",
  "dependencies",
  "devDependencies",
  "dependenciesMeta",
  "peerDependencies",
  "peerDependenciesMeta",
  "optionalDependencies",
  "bundledDependencies",
  "bundleDependencies",
  "extensionPack",
  "extensionDependencies",
  "flat",
  "packageManager",
  "engines",
  "engineStrict",
  "volta",
  "languageName",
  "os",
  "cpu",
  "preferGlobal",
  "publishConfig",
  "icon",
  "badges",
  "galleryBanner",
  "preview",
  "markdown",
  "pnpm",
];

pub fn sort_package_json(input: &str) -> Result<String, serde_json::Error> {
  let value: Value = serde_json::from_str(input)?;

  let sorted_value = if let Value::Object(obj) = value {
    Value::Object(sort_object_keys(obj))
  } else {
    value
  };

  serde_json::to_string_pretty(&sorted_value)
}

fn sort_object_alphabetically(obj: &Map<String, Value>) -> Map<String, Value> {
  let mut keys: Vec<&String> = obj.keys().collect();
  keys.sort();

  let mut result = Map::new();
  for key in keys {
    if let Some(value) = obj.get(key) {
      result.insert(key.clone(), value.clone());
    }
  }
  result
}

fn sort_object_recursive(obj: &Map<String, Value>) -> Map<String, Value> {
  let mut keys: Vec<&String> = obj.keys().collect();
  keys.sort();

  let mut result = Map::new();
  for key in keys {
    if let Some(value) = obj.get(key) {
      let transformed_value = match value {
        Value::Object(nested) => Value::Object(sort_object_recursive(nested)),
        _ => value.clone(),
      };
      result.insert(key.clone(), transformed_value);
    }
  }
  result
}

fn sort_object_keys(obj: Map<String, Value>) -> Map<String, Value> {
  let mut result = Map::new();
  let mut remaining_keys: Vec<String> = obj.keys().cloned().collect();

  // Process fields in FIELDS_ORDER with inline transformations
  for &field in FIELDS_ORDER {
    if let Some(value) = obj.get(field) {
      let transformed = match (field, value) {
        // Dependency-like fields - alphabetically sorted
        (
          "dependencies" | "devDependencies" | "peerDependencies" | "optionalDependencies"
          | "resolutions" | "overrides" | "engines" | "publishConfig" | "config" | "bin",
          Value::Object(obj),
        ) => Value::Object(sort_object_alphabetically(obj)),

        // Config objects - recursively sorted
        (
          "babel" | "jest" | "mocha" | "nyc" | "c8" | "ava" | "eslintConfig" | "prettier"
          | "stylelint" | "nodemonConfig" | "browserify" | "xo" | "husky" | "commitlint"
          | "remarkConfig" | "volta" | "oclif",
          Value::Object(obj),
        ) => Value::Object(sort_object_recursive(obj)),

        // No transformation needed
        _ => value.clone(),
      };

      result.insert(field.to_string(), transformed);
      remaining_keys.retain(|k| k != field);
    }
  }

  // Separate remaining keys into non-private and private
  let mut non_private: Vec<String> = Vec::new();
  let mut private: Vec<String> = Vec::new();

  for key in remaining_keys {
    if key.starts_with('_') {
      private.push(key);
    } else {
      non_private.push(key);
    }
  }

  // Sort non-private keys alphabetically
  non_private.sort();
  for key in non_private {
    if let Some(value) = obj.get(&key) {
      result.insert(key, value.clone());
    }
  }

  // Sort private keys alphabetically
  private.sort();
  for key in private {
    if let Some(value) = obj.get(&key) {
      result.insert(key, value.clone());
    }
  }

  result
}
