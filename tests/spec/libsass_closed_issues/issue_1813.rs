//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1813.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1813")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function foo($value) {\
             \n  $a: bar($value);\
             \n  @return $value;\
             \n}\n\
             \n@function bar($list) {\
             \n  @while (true) {\
             \n    @return true;\
             \n  }\
             \n}\n\
             \na {\
             \n  b: foo(true);\
             \n}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
