//! Tests auto-converted from "sass-spec/spec/libsass/basic"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass/basic/44_not_number_expression.hrx"
#[test]
fn t44_not_number_expression() {
    assert_eq!(
        rsass(
            "@if 2 {\
            \n  div {\
            \n    background: green;\
            \n  }\
            \n}\
            \n@if not 2 {\
            \n  div {\
            \n    background: red;\
            \n  }\
            \n}\
            \n@if not not 2 {\
            \n  div {\
            \n    background: blue;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  background: green;\
        \n}\
        \ndiv {\
        \n  background: blue;\
        \n}\
        \n"
    );
}
