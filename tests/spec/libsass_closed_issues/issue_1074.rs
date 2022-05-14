//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1074.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1074")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$i: 1;\
             \n.foo#{-$i} { a:b }\
             \n.foo-#{$i} { a:b }\
             \n.foo#{-1} { a:b }\
             \n.foo-#{1} { a:b }\n"),
        ".foo-1 {\
         \n  a: b;\
         \n}\
         \n.foo-1 {\
         \n  a: b;\
         \n}\
         \n.foo-1 {\
         \n  a: b;\
         \n}\
         \n.foo-1 {\
         \n  a: b;\
         \n}\n"
    );
}
