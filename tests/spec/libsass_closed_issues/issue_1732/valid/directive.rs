//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1732/valid/directive.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@media all {\
            \n  .foo {\
            \n\tcolor: red;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "@media all {\
        \n  .foo {\
        \n    color: red;\
        \n  }\
        \n}\
        \n"
    );
}
