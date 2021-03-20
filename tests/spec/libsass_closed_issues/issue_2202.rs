//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2202.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@customAtRule;\r\
            \ntest {\r\
            \n  content: bar\r\
            \n}"
        )
        .unwrap(),
        "@customAtRule;\
        \ntest {\
        \n  content: bar;\
        \n}\
        \n"
    );
}
