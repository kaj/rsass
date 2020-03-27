//! Tests auto-converted from "sass-spec/spec/directives/function"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/directives/function/escaped.hrx"
#[test]
fn escaped() {
    assert_eq!(
        rsass(
            "// Function names can be defined and referred to using escapes, which are\
            \n// normalized.\
            \n@function f\\6Fo-bar() {@return 1}\
            \n\
            \na {b: foo-b\\61r()}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
