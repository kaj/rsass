//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_701.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_701")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\n\
             \n.test-1 {\
             \n  content: null;\
             \n  content: meta.inspect(null);\
             \n  content: meta.inspect(false);\
             \n  content: meta.inspect(true);\
             \n  content: meta.inspect(42);\
             \n  content: meta.inspect(42.3);\
             \n  content: meta.inspect(42px);\
             \n  content: meta.inspect(\"string\");\
             \n  $list: 1, 2, 3;\
             \n  content: meta.inspect($list);\
             \n  $map: ( a: 1, b: 2, c: 3 );\
             \n  content: meta.inspect($map);\
             \n}\n"),
        ".test-1 {\
         \n  content: null;\
         \n  content: false;\
         \n  content: true;\
         \n  content: 42;\
         \n  content: 42.3;\
         \n  content: 42px;\
         \n  content: \"string\";\
         \n  content: 1, 2, 3;\
         \n  content: (a: 1, b: 2, c: 3);\
         \n}\n"
    );
}
