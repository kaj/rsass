//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1459.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@font-face {\r\
            \n  font-family: \"Font Name\";\r\
            \n  src: local(\"Arial\");\r\
            \n  unicode-range: U+270C;\r\
            \n}"
        )
        .unwrap(),
        "@font-face {\
        \n  font-family: \"Font Name\";\
        \n  src: local(\"Arial\");\
        \n  unicode-range: U+270C;\
        \n}\
        \n"
    );
}
