//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_100.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$endColor: red;\r\
            \ntest {\r\
            \n  background-color: darken($endColor, 10%) \\9;\r\
            \n}"
        )
        .unwrap(),
        "test {\
        \n  background-color: #cc0000 \\9 ;\
        \n}\
        \n"
    );
}
