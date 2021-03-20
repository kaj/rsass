//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_rule_comments.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "/* Foo\
            \n * Bar */\
            \n.foo {\
            \n  a: b; }\
            \n"
        )
        .unwrap(),
        "/* Foo\
        \n * Bar */\
        \n.foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}
