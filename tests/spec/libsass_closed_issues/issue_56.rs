//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_56.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@media (min-width: 980px) {\r\
            \n    a {\r\
            \n        color: red;\r\
            \n    }\r\
            \n}"
        )
        .unwrap(),
        "@media (min-width: 980px) {\
        \n  a {\
        \n    color: red;\
        \n  }\
        \n}\
        \n"
    );
}
