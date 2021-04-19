//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/44_selector/todo_single_escape/21_escaped_interpolated_variable.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "$key: \'bar\';\
             \n$suffix1: \'\\@#{$key}\';\
             \n.test21#{$suffix1} { content: \'2.1\'; }\
             \n"
        )
        .unwrap_err(),
        "Error: expected selector.\
         \n  ,\
         \n3 | .test21@bar{ content: \'2.1\'; }\
         \n  |        ^\
         \n  \'\
         \n  input.scss 3:8  root stylesheet\
         \n",
    );
}
