//! Tests auto-converted from "sass-spec/spec/core_functions"
//! version 6c9bd98c, 2019-02-13 14:56:19 -0800.
//! See <https://github.com/sass/sass-spec> for source material.\n
use rsass::{compile_scss, OutputStyle};

mod color;

mod content_exists;

/// From "sass-spec/spec/core_functions/feature_exists.hrx"
#[test]
#[ignore] // failing
fn feature_exists() {
    assert_eq!(
        rsass(
            ".feature-exists {\n  custom-property: feature-exists(custom-property);\n}"
        )
        .unwrap(),
        ".feature-exists {\n  custom-property: true;\n}\n"
    );
}

mod get_function;

// Ignoring "inspect.hrx", not a single spec.

mod invert;

/// From "sass-spec/spec/core_functions/is_bracketed.hrx"
#[test]
fn is_bracketed() {
    assert_eq!(
        rsass(
            ".is-bracketed {\n  unbracketed-empty: is-bracketed(());\n  unbracketed-singleton: is-bracketed(foo);\n  unbracketed-multiple: is-bracketed(foo bar);\n  bracketed-empty: is-bracketed([]);\n  bracketed-singleton: is-bracketed([foo]);\n  bracketed-multiple: is-bracketed([foo bar]);\n}"
        )
        .unwrap(),
        ".is-bracketed {\n  unbracketed-empty: false;\n  unbracketed-singleton: false;\n  unbracketed-multiple: false;\n  bracketed-empty: true;\n  bracketed-singleton: true;\n  bracketed-multiple: true;\n}\n"
    );
}

mod join;

fn rsass(input: &str) -> Result<String, String> {
    compile_scss(input.as_bytes(), OutputStyle::Expanded)
        .map_err(|e| format!("rsass failed: {}", e))
        .and_then(|s| {
            String::from_utf8(s)
                .map(|s| s.replace("\n\n", "\n"))
                .map_err(|e| format!("{:?}", e))
        })
}
