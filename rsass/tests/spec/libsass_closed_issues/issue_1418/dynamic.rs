//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1418/dynamic.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("dynamic")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "@use \"sass:meta\";\
             \nfoo {\
             \n    color: meta.call(missing, $a: b);\
             \n}\n"
        ),
        "DEPRECATION WARNING [call-string]: Passing a string to call() is deprecated and will be illegal in Dart Sass 2.0.0.\n\
         \nRecommendation: call(get-function(missing))\n\
         \n  ,\
         \n3 |     color: meta.call(missing, $a: b);\
         \n  |            ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 3:12  root stylesheet\n\
         \nError: Plain CSS functions don\'t support keyword arguments.\
         \n  ,\
         \n3 |     color: meta.call(missing, $a: b);\
         \n  |            ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:12  root stylesheet",
    );
}
