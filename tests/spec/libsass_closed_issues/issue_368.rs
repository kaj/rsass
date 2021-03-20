//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_368.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@if true {\
            \n  div {\
            \n    background: green;\
            \n  }\
            \n}\
            \n@if not true {\
            \n  div {\
            \n    background: red;\
            \n  }\
            \n}\
            \n@if not not true {\
            \n  div {\
            \n    background: blue;\
            \n  }\
            \n}\
            \n@if not (true or false) {\
            \n  div {\
            \n    background: black;\
            \n  }\
            \n}"
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
