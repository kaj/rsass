//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1253.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$foo: bar;\
            \n@keyframes $foo {\
            \n  from { a: b }\
            \n  to { a: c }\
            \n}"
        )
        .unwrap(),
        "@keyframes $foo {\
        \n  from {\
        \n    a: b;\
        \n  }\
        \n  to {\
        \n    a: c;\
        \n  }\
        \n}\
        \n"
    );
}
