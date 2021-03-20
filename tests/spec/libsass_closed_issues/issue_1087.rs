//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1087.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$foo: bar;\
            \na {\
            \n  foo: url($foo);\
            \n  foo: url(#{$foo});\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  foo: url(bar);\
        \n  foo: url(bar);\
        \n}\
        \n"
    );
}
