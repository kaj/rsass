//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1170/eval.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
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
