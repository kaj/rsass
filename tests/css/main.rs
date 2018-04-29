//! Tests from `sass_spec/spec/css/`

extern crate rsass;
use rsass::{compile_scss, OutputStyle};

mod unicode_range;
mod unknown_directive;

#[test]
fn bizarrely_formatted_comments() {
    check(
        ".foo {\n    /* Foo\n Bar\nBaz */\n  a: b; }\n",
        ".foo {\n  /* Foo\n Bar\nBaz */\n  a: b;\n}\n",
    )
}


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

#[test]
fn selector_slotted_part() {
    check(
        "::slotted(.a) {x: y}\n\n\
         ::slotted(.c.d) {x: y}\n\
         ::slotted(.f) {x: y}\n",
        "::slotted(.a) {\n  x: y;\n}\n\n\
         ::slotted(.c.d) {\n  x: y;\n}\n\n\
         ::slotted(.f) {\n  x: y;\n}\n",
    )
}

#[test]
#[ignore] // @extend is not yet supported
fn selector_slotted() {
    check(
        "::slotted(.a) {x: y}\n\n\
         ::slotted(.c.d) {x: y}\n\
         .e {@extend .c}\n\n\
         ::slotted(.f) {x: y}\n\
         ::slotted(.g) {@extend .f}\n",
        "::slotted(.a) {\n  x: y;\n}\n\n\
         ::slotted(.c.d, .d.e) {\n  x: y;\n}\n\
         ::slotted(.f, ::slotted(.g)) {\n  x: y;\n}\n",
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
