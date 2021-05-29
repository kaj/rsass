//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1266/max.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "$foo: 1 2 3 blah 4;\
             \nfoo {\
             \n  bar: call(max, $foo...);\
             \n}\n"
        ),
        "DEPRECATION WARNING: Passing a string to call() is deprecated and will be illegal in Dart Sass 2.0.0.\n\
         \nRecommendation: call(get-function(max))\n\
         \n  ,\
         \n3 |   bar: call(max, $foo...);\
         \n  |        ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 3:8  root stylesheet\n\
         \nError: blah is not a number.\
         \n  ,\
         \n3 |   bar: call(max, $foo...);\
         \n  |        ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:8  root stylesheet",
    );
}
