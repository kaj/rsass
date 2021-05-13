//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1169/simple.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$map1: (\r\
             \n   red: \'bar\',\r\
             \n  \'red\': \'foo\',\r\
             \n);\r\
             \n\r\
             \n$map2: (\r\
             \n   red: \'bar\',\r\
             \n  \'red\': #{red},\r\
             \n);\r\
             \n\r\
             \n.foo {\r\
             \n  content: inspect($map1);\r\
             \n  content: inspect($map2);\r\
             \n}"),
        ".foo {\
         \n  content: (red: \"bar\", \"red\": \"foo\");\
         \n  content: (red: \"bar\", \"red\": red);\
         \n}\n"
    );
}
