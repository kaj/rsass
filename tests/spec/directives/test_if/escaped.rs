//! Tests auto-converted from "sass-spec/spec/directives/if/escaped.hrx"

#[test]
fn if_only() {
    assert_eq!(
        crate::rsass(
            "// Escapes should be normalized before directives are parsed.\
            \n@\\69 f true {a {b: c}}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \n"
    );
}
#[test]
fn with_else() {
    assert_eq!(
        crate::rsass(
            "// See sass/dart-sass#1011\
            \n@if false {}\
            \n@\\65lse {a {b: c}}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \n"
    );
}
