//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1940.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1940")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:map\";\
             \n// ----\r\
             \n// libsass (v3.3.2)\r\
             \n// ----\r\
             \n\r\
             \n$prefix:(\r\
             \n\tfoo: foo,\r\
             \n\tbar: #bar\r\
             \n);\r\
             \n\r\
             \n#{map.get($prefix, foo)} {\r\
             \n  color: red;\r\
             \n}\r\
             \n#{map.get($prefix, bar)} {\r\
             \n  color: red;\r\
             \n}\r\
             \n\r\
             \n#{map.get($prefix, foo)} .baz {\r\
             \n  color: red;\r\
             \n}\r\
             \n#{map.get($prefix, bar)} .baz {\r\
             \n  color: red;\r\
             \n}"),
        "foo {\
         \n  color: red;\
         \n}\
         \n#bar {\
         \n  color: red;\
         \n}\
         \nfoo .baz {\
         \n  color: red;\
         \n}\
         \n#bar .baz {\
         \n  color: red;\
         \n}\n"
    );
}
