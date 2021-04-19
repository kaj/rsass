//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/44_selector/todo_single_escape/31_escaped_literal.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            ".test31#{\'\\@baz\'} { content: \'3.1\'; }\
             \n"
        )
        .unwrap_err(),
        "Error: expected selector.\
         \n  ,\
         \n1 | .test31@baz{ content: \'3.1\'; }\
         \n  |        ^\
         \n  \'\
         \n  input.scss 1:8  root stylesheet\
         \n",
    );
}
