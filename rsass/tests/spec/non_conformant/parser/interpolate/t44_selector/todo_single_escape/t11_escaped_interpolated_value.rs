//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/44_selector/todo_single_escape/11_escaped_interpolated_value.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("11_escaped_interpolated_value")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "$key: \'bar\';\
             \n.test11#{\'\\@#{$key}\'} { content: \'1.1\'; }\n"
        ),
        "Error: expected selector.\
         \n  ,--> input.scss\
         \n2 | .test11#{\'\\@#{$key}\'} { content: \'1.1\'; }\
         \n  |          ^^^^^^^^^^^ \
         \n  \'\
         \n  ,\
         \n1 | .test11@bar \
         \n  |        = error in interpolated output\
         \n  \'\
         \n  input.scss 2:10  root stylesheet",
    );
}
