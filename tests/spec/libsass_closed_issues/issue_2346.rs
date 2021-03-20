//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2346.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "$items: 3;\r\
            \nli {\r\
            \n  &:nth-child(#{$items}n - #{$items}) {\r\
            \n    color: red;\r\
            \n  }\r\
            \n}"
        )
        .unwrap(),
        "li:nth-child(3n-3) {\
        \n  color: red;\
        \n}\
        \n"
    );
}
