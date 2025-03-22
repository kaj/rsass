//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/133_test_nested_function_def.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("133_test_nested_function_def")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  @function foo() {@return 1}\
             \n  a: foo(); }\n\
             \nbar {b: foo()}\n"),
        "foo {\
         \n  a: 1;\
         \n}\
         \nbar {\
         \n  b: foo();\
         \n}\n"
    );
}
