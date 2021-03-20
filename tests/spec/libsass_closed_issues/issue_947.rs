//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_947.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@keyframes test {\
            \n  $var: 10%;\
            \n  #{$var} {\
            \n    color: red;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@keyframes test {\
        \n  10% {\
        \n    color: red;\
        \n  }\
        \n}\
        \n"
    );
}
