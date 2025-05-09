//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/basic_function.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("basic_function")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function foo() {\
             \n  @return 1 + 2;\
             \n}\n\
             \nbar {\
             \n  a: foo();\
             \n}\n"),
        "bar {\
         \n  a: 3;\
         \n}\n"
    );
}
