//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2034.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ":not(.thing) {\r\
            \n    color: red;\r\
            \n}\r\
            \n:not(.bar) {\r\
            \n    @extend .thing;\r\
            \n    background: blue;\r\
            \n}"
        )
        .unwrap(),
        ":not(.thing) {\
        \n  color: red;\
        \n}\
        \n:not(.bar) {\
        \n  background: blue;\
        \n}\
        \n"
    );
}
