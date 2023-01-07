//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_873.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_873")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$quoted: \"notification\";\
             \n$unquoted: notification;\n\
             \n@function func($var) {\
             \n  @return $var;\
             \n}\n\
             \nfoo {\
             \n  foo: func(notification);\
             \n  foo: #{notification};\
             \n  foo: #{$quoted};\
             \n  foo: $quoted;\
             \n  foo: #{$unquoted};\
             \n  foo: $unquoted;\
             \n}\n"),
        "foo {\
         \n  foo: notification;\
         \n  foo: notification;\
         \n  foo: notification;\
         \n  foo: \"notification\";\
         \n  foo: notification;\
         \n  foo: notification;\
         \n}\n"
    );
}
