//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1063.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "%foo {\
            \n  & > x { display: block; }\
            \n}\
            \n\
            \na {\
            \n  > b { @extend %foo; }\
            \n  > b > c { @extend %foo; }\
            \n}\
            \n"
        )
        .unwrap(),
        "a > b > c > x, a > b > x {\
        \n  display: block;\
        \n}\
        \n"
    );
}
