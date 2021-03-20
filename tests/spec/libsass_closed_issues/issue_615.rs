//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_615.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "$foo: \"bar\";\
            \n%#{\"foo--#{$foo}\"} {\
            \n  foo: bar;\
            \n}\
            \n\
            \na {\
            \n  @extend %foo--bar;\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  foo: bar;\
        \n}\
        \n"
    );
}
