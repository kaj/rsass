//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_221289.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\r\
            \n  bar: if(0,0<0,0);\r\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  bar: false;\
        \n}\
        \n"
    );
}
