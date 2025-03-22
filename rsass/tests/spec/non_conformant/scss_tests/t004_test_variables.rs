//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/004_test_variables.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("004_test_variables")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  $var: 2;\
             \n  $another-var: 4;\
             \n  a: $var;\
             \n  b: $var + $another-var;}\n"),
        "foo {\
         \n  a: 2;\
         \n  b: 6;\
         \n}\n"
    );
}
