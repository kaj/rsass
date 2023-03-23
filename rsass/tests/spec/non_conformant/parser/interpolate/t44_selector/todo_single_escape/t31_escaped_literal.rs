//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/44_selector/todo_single_escape/31_escaped_literal.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("31_escaped_literal")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(".test31#{\'\\@baz\'} { content: \'3.1\'; }\n"),
        "Error: expected selector.\
         \n  ,--> input.scss\
         \n1 | .test31#{\'\\@baz\'} { content: \'3.1\'; }\
         \n  |          ^^^^^^^ \
         \n  \'\
         \n  ,\
         \n1 | .test31@baz \
         \n  |        = error in interpolated output\
         \n  \'\
         \n  input.scss 1:10  root stylesheet",
    );
}
