//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue-2681.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "%button-styles {\
            \n  color: red;\
            \n\
            \n  &:focus {\
            \n    color: blue;\
            \n  }\
            \n}\
            \n\
            \n[type=\"button\"] {\
            \n  @extend %button-styles;\
            \n}\
            \n\
            \n"
        )
        .unwrap(),
        "[type=button] {\
        \n  color: red;\
        \n}\
        \n[type=button]:focus {\
        \n  color: blue;\
        \n}\
        \n"
    );
}
