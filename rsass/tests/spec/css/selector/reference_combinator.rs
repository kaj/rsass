//! Tests auto-converted from "sass-spec/spec/css/selector/reference_combinator.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("reference_combinator")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "// Reference combinators used to be supported by Sass when they were part of the\
             \n// CSS spec, but they\'re no longer supported and should now produce errors.\
             \n.foo /bar/ .baz {\
             \n  a: b;\
             \n}\n"
        ),
        "Error: expected selector.\
         \n  ,\
         \n3 | .foo /bar/ .baz{\
         \n  |      ^\
         \n  \'\
         \n  input.scss 3:6  root stylesheet",
    );
}
