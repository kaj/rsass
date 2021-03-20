//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2303.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".wrapper-class {\r\
            \n  @import \'module\';\r\
            \n}"
        )
        .unwrap(),
        ".wrapper-class .okay {\
        \n  background: green;\
        \n}\
        \n.wrapper-class .broken {\
        \n  background: red;\
        \n}\
        \n"
    );
}
