//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1647"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_1647/directives.hrx"
#[test]
fn directives() {
    assert_eq!(
        rsass(
            "@foo #{\"directive\"} {\
            \n  .#{\"foo\"} { #{\"foo-prop\"}: #{\"foo-val\"}; }\
            \n}\
            \n"
        )
        .unwrap(),
        "@foo directive {\
        \n  .foo {\
        \n    foo-prop: foo-val;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1647/selectors.hrx"
#[test]
fn selectors() {
    assert_eq!(
        rsass(
            "$map: (foo: \'b\', bar: c);\
            \n$list: (\'d\', e);\
            \n\
            \na {\
            \n  #{map-get($map, foo)} & {\
            \n      foo: bar;\
            \n  }\
            \n  #{map-get($map, bar)} & {\
            \n      foo: bar;\
            \n  }\
            \n\
            \n  #{nth($list, 1)} & {\
            \n      foo: bar;\
            \n  }\
            \n\
            \n  #{nth($list, 2)} & {\
            \n      foo: bar;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "b a {\
        \n  foo: bar;\
        \n}\
        \nc a {\
        \n  foo: bar;\
        \n}\
        \nd a {\
        \n  foo: bar;\
        \n}\
        \ne a {\
        \n  foo: bar;\
        \n}\
        \n"
    );
}
