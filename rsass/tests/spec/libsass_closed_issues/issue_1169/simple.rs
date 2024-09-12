//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1169/simple.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("simple")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\n\
             \n$map1: (\
             \n   red: \'bar\',\
             \n  \'red\': \'foo\',\
             \n);\n\
             \n$map2: (\
             \n   red: \'bar\',\
             \n  \'red\': #{red},\
             \n);\n\
             \n.foo {\
             \n  content: meta.inspect($map1);\
             \n  content: meta.inspect($map2);\
             \n}"),
        ".foo {\
         \n  content: (red: \"bar\", \"red\": \"foo\");\
         \n  content: (red: \"bar\", \"red\": red);\
         \n}\n"
    );
}
