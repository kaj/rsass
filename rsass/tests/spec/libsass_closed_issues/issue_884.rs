//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_884.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_884")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function foo() {\
             \n  @return 2;\
             \n}\n\
             \n$foo: false;\
             \n@if foo() % 2 == 0 {\
             \n  $foo: true;\
             \n}\n\
             \na {\
             \n  foo: $foo;\
             \n}\n"),
        "a {\
         \n  foo: true;\
         \n}\n"
    );
}
