//! Tests auto-converted from "sass-spec/spec/css/functions/min_max/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // missing error
fn dangling_operator() {
    assert_eq!(
        runner().err(
            "// A dangling operator is a syntax error in both syntaxes, so it should fail to\
             \n// compile.\
             \n.dangling-operator {\
             \n  x: min(1px +, 2px);\
             \n}\n"
        ),
        "Error: Expected expression.\
         \n  ,\
         \n4 |   x: min(1px +, 2px);\
         \n  |               ^\
         \n  \'\
         \n  input.scss 4:15  root stylesheet",
    );
}
#[test]
fn plain_css_function() {
    assert_eq!(
        runner().err(
            "// A plain CSS function causes min() to be parsed as Sass, and then fail at\
             \n// runtime because the argument isn\'t a number.\
             \n.plain-css-function {\
             \n  x: min(something(1px), 2px);\
             \n}\n"
        ),
        "Error: something(1px) is not a number.\
         \n  ,\
         \n4 |   x: min(something(1px), 2px);\
         \n  |      ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 4:6  root stylesheet",
    );
}
