//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/44_selector/todo_single_escape/31_escaped_literal.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(".test31#{\'\\@baz\'} { content: \'3.1\'; }\n"),
        "Error: expected selector.\
         \n  ,\
         \n1 | .test31@baz{ content: \'3.1\'; }\
         \n  |        ^\
         \n  \'\
         \n  input.scss 1:8  root stylesheet",
    );
}
