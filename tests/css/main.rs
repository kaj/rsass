//! Tests from `sass_spec/spec/css/`

extern crate rsass;
use rsass::{compile_scss, OutputStyle};

mod unicode_range;
mod unknown_directive;

#[test]
fn multi_star_comments() {
    // Note, actual expected has single newlines, but the sass-spec
    // test runner succeeds my implementation.
    check(
        "a /***/ b {x: y}\n\
         a /****/ b {x: y}\n\
         a /* **/ b {x: y}\n\
         a /** */ b {x: y}\n",
        "a b {\n  x: y;\n}\n\n\
         a b {\n  x: y;\n}\n\n\
         a b {\n  x: y;\n}\n\n\
         a b {\n  x: y;\n}\n",
    )
}

fn check(input: &str, expected: &str) {
    assert_eq!(
        compile_scss(input.as_bytes(), OutputStyle::Expanded)
            .and_then(|s| Ok(String::from_utf8(s)?))
            .unwrap(),
        expected
    );
}
