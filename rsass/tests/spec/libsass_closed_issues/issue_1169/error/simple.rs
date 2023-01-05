//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1169/error/simple.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("simple")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$map: (\r\
             \n  red: \'bar\',\r\
             \n  #{red}: \'baz\',\r\
             \n);\r\
             \n\r\
             \n.foo {\r\
             \n  content: inspect($map);\r\
             \n}"),
        ".foo {\
         \n  content: (red: \"bar\", red: \"baz\");\
         \n}\n"
    );
}
