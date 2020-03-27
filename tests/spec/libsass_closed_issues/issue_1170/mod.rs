//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1170"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_1170/eval.hrx"
#[test]
fn eval() {
    assert_eq!(
        rsass(
            "el {\
            \n  @if ((& + \'\') == \'el\') {\
            \n    content: \"It works!\";\
            \n  }\
            \n}"
        )
        .unwrap(),
        "el {\
        \n  content: \"It works!\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1170/parse.hrx"
#[test]
fn parse() {
    assert_eq!(
        rsass(
            "el {\
            \n  @if (& + \'\' == \'el\') {\
            \n    content: \"It works!\";\
            \n  }\
            \n}"
        )
        .unwrap(),
        "el {\
        \n  content: \"It works!\";\
        \n}\
        \n"
    );
}
