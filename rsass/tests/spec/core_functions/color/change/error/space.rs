//! Tests auto-converted from "sass-spec/spec/core_functions/color/change/error/space.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("space")
}

#[test]
#[ignore] // wrong error
fn quoted() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.change(red, $space: \"lab\")}\n"
        ),
        "Error: $space: Expected \"lab\" to be an unquoted string.\
         \n  ,\
         \n2 | a {b: color.change(red, $space: \"lab\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn unknown() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.change(red, $space: c)}\n"
        ),
        "Error: $space: Unknown color space \"c\".\
         \n  ,\
         \n2 | a {b: color.change(red, $space: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
