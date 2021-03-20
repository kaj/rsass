//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_131.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$foo: bar;\r\
            \n\r\
            \ndiv {\r\
            \n    content: \"foo #{$foo}\"\r\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  content: \"foo bar\";\
        \n}\
        \n"
    );
}
