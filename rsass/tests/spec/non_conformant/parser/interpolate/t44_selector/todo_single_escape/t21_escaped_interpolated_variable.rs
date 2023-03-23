//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/44_selector/todo_single_escape/21_escaped_interpolated_variable.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("21_escaped_interpolated_variable")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "$key: \'bar\';\
             \n$suffix1: \'\\@#{$key}\';\
             \n.test21#{$suffix1} { content: \'2.1\'; }\n"
        ),
        "Error: expected selector.\
         \n  ,--> input.scss\
         \n3 | .test21#{$suffix1} { content: \'2.1\'; }\
         \n  |          ^^^^^^^^ \
         \n  \'\
         \n  ,\
         \n1 | .test21@bar \
         \n  |        = error in interpolated output\
         \n  \'\
         \n  input.scss 3:10  root stylesheet",
    );
}
