//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/feature-queries"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/scss/feature-queries/basic.hrx"
#[test]
fn basic() {
    assert_eq!(
        rsass(
            "@supports (foo: bar) {\
            \n   div {\
            \n     foo: bar;\
            \n   }\
            \n }\
            \n @supports not (foo: bar) {\
            \n   div {\
            \n     bar: baz;\
            \n   }\
            \n }\
            \n @supports (foo: bar) and (bar: baz) {\
            \n   div {\
            \n     foo: bar;\
            \n     bar: baz;\
            \n   }\
            \n }\
            \n @supports (foo: bar) or (bar: baz) {\
            \n   div {\
            \n     bar: baz;\
            \n   }\
            \n }\
            \n"
        )
        .unwrap(),
        "@supports (foo: bar) {\
        \n  div {\
        \n    foo: bar;\
        \n  }\
        \n}\
        \n@supports not (foo: bar) {\
        \n  div {\
        \n    bar: baz;\
        \n  }\
        \n}\
        \n@supports (foo: bar) and (bar: baz) {\
        \n  div {\
        \n    foo: bar;\
        \n    bar: baz;\
        \n  }\
        \n}\
        \n@supports (foo: bar) or (bar: baz) {\
        \n  div {\
        \n    bar: baz;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/feature-queries/nested.hrx"
#[test]
fn nested() {
    assert_eq!(
        rsass(
            ".foo {\
            \n     display: block;\
            \n     @supports (display: flex) {\
            \n         display: flex;\
            \n     }\
            \n }\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  display: block;\
        \n}\
        \n@supports (display: flex) {\
        \n  .foo {\
        \n    display: flex;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/feature-queries/without-query.hrx"

// Ignoring "without_query", error tests are not supported yet.
