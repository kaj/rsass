//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2074.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function calc_foo() {\r\
            \n  @return \"bar\";\r\
            \n}\r\
            \nfoo {\r\
            \n  test1: calc-foo();\r\
            \n  test2: calc_foo();\r\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  test1: \"bar\";\
        \n  test2: \"bar\";\
        \n}\
        \n"
    );
}
