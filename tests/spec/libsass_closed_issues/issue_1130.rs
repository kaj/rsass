//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1130.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function foo($args...) {\
            \n  @return bar($args...);\
            \n}\
            \n\
            \n@function bar() {\
            \n @return \"hi\";\
            \n}\
            \n\
            \n.foo {\
            \n  result: foo();\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  result: \"hi\";\
        \n}\
        \n"
    );
}
