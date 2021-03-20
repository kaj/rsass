//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/strings.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  content: blang + 1;\
            \n  content: 1 + blang;\
            \n  content: \"blang\" + 1;\
            \n  content: 1 + \"blang\";\
            \n  content: bar + \"foo\";\
            \n  content: \"quoted\" + unquoted;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  content: blang1;\
        \n  content: 1blang;\
        \n  content: \"blang1\";\
        \n  content: \"1blang\";\
        \n  content: barfoo;\
        \n  content: \"quotedunquoted\";\
        \n}\
        \n"
    );
}
