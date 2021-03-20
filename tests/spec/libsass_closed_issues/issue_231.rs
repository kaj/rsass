//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_231.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "// test.scss:\r\
            \na {\r\
            \n  background-image: url(fn(\"s\"));\r\
            \n}"
        )
        .unwrap(),
        "a {\
        \n  background-image: url(fn(\"s\"));\
        \n}\
        \n"
    );
}
