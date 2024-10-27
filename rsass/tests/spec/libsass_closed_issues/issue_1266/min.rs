//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1266/min.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("min")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@use \"sass:meta\";\
             \n$foo: 1 2 3 blah 4;\
             \nfoo {\
             \n  bar: meta.call(min, $foo...);\
             \n}\n"
        ),
        "DEPRECATION WARNING: Passing a string to call() is deprecated and will be illegal in Dart Sass 2.0.0.\n\
         \nRecommendation: call(get-function(min))\n\
         \n  ,\
         \n4 |   bar: meta.call(min, $foo...);\
         \n  |        ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 4:8  root stylesheet\n\
         \nDEPRECATION WARNING: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse math.min instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n4 |   bar: meta.call(min, $foo...);\
         \n  |        ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 4:8  root stylesheet\n\
         \nError: blah is not a number.\
         \n  ,\
         \n4 |   bar: meta.call(min, $foo...);\
         \n  |        ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 4:8  root stylesheet",
    );
}
