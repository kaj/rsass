//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1169/error/simple.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("simple")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\n\
             \n$map: (\
             \n  red: \'bar\',\
             \n  #{red}: \'baz\',\
             \n);\n\
             \n.foo {\
             \n  content: meta.inspect($map);\
             \n}"),
        ".foo {\
         \n  content: (red: \"bar\", red: \"baz\");\
         \n}\n"
    );
}
