//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1654/basic.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "%foo {\
            \n  &bar {\
            \n    display: block;\
            \n  }\
            \n  &.bar {\
            \n    display: block;\
            \n  }\
            \n}\
            \nzoo {\
            \n  @extend %foo;\
            \n}"
        )
        .unwrap(),
        "zoo.bar {\
        \n  display: block;\
        \n}\
        \n"
    );
}
