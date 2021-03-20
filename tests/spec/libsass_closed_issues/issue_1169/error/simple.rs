//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1169/error/simple.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$map: (\r\
            \n  red: \'bar\',\r\
            \n  #{red}: \'baz\',\r\
            \n);\r\
            \n\r\
            \n.foo {\r\
            \n  content: inspect($map);\r\
            \n}"
        )
        .unwrap(),
        ".foo {\
        \n  content: (red: \"bar\", red: \"baz\");\
        \n}\
        \n"
    );
}
