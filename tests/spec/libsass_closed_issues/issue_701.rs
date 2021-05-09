//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_701.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".test-1 {\
             \n  content: null;\
             \n  content: inspect(null);\
             \n  content: inspect(false);\
             \n  content: inspect(true);\
             \n  content: inspect(42);\
             \n  content: inspect(42.3);\
             \n  content: inspect(42px);\
             \n  content: inspect(\"string\");\
             \n  $list: 1, 2, 3;\
             \n  content: inspect($list);\
             \n  $map: ( a: 1, b: 2, c: 3 );\
             \n  content: inspect($map);\
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
