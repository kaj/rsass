//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/063_test_function_args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function plus($var1, $var2) {\
             \n  @return $var1 + $var2;\
             \n}\n\
             \nbar {\
             \n  a: plus(1, 2);\
             \n}\n"),
        "bar {\
         \n  a: 3;\
         \n}\n"
    );
}
