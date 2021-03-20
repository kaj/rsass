//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2295/basic.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".my-scope {\r\
            \n  @import \'include.scss\';\r\
            \n}"
        )
        .unwrap(),
        ".my-scope .foo {\
        \n  display: none;\
        \n}\
        \n"
    );
}
