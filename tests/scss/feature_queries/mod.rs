//! Tests auto-converted from "sass-spec/spec/scss/feature-queries"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

/// From "sass-spec/spec/scss/feature-queries/basic"
#[test]
fn basic() {
    assert_eq!(
        rsass(
            "@supports (foo: bar) {\n   div {\n     foo: bar;\n   }\n }\n @supports not (foo: bar) {\n   div {\n     bar: baz;\n   }\n }\n @supports (foo: bar) and (bar: baz) {\n   div {\n     foo: bar;\n     bar: baz;\n   }\n }\n @supports (foo: bar) or (bar: baz) {\n   div {\n     bar: baz;\n   }\n }\n"
        )
        .unwrap(),
        "@supports (foo: bar) {\n  div {\n    foo: bar;\n  }\n}\n@supports not (foo: bar) {\n  div {\n    bar: baz;\n  }\n}\n@supports (foo: bar) and (bar: baz) {\n  div {\n    foo: bar;\n    bar: baz;\n  }\n}\n@supports (foo: bar) or (bar: baz) {\n  div {\n    bar: baz;\n  }\n}\n"
    );
}

/// From "sass-spec/spec/scss/feature-queries/nested"
#[test]
fn nested() {
    assert_eq!(
        rsass(
            ".foo {\n     display: block;\n     @supports (display: flex) {\n         display: flex;\n     }\n }\n"
        )
        .unwrap(),
        ".foo {\n  display: block;\n}\n@supports (display: flex) {\n  .foo {\n    display: flex;\n  }\n}\n"
    );
}

// Ignoring "without-query", tests with expected error not implemented yet.
