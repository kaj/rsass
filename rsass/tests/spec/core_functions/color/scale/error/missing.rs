//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/error/missing.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("missing")
}

#[test]
#[ignore] // wrong error
fn alpha() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(rgb(0 0 0 / none), $alpha: 10%)}\n"
        ),
        "Error: $alpha: Because the CSS working group is still deciding on the best behavior, Sass doesn\'t currently support modifying missing channels (color: rgb(0 0 0 / none)).\
         \n  ,\
         \n2 | a {b: color.scale(rgb(0 0 0 / none), $alpha: 10%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn legacy() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(rgb(none 0 0), $red: 10%)}\n"
        ),
        "Error: $red: Because the CSS working group is still deciding on the best behavior, Sass doesn\'t currently support modifying missing channels (color: rgb(none 0 0)).\
         \n  ,\
         \n2 | a {b: color.scale(rgb(none 0 0), $red: 10%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn modern() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(lab(none 0 0), $lightness: 10%, $space: lab)}\n"
        ),
        "Error: $lightness: Because the CSS working group is still deciding on the best behavior, Sass doesn\'t currently support modifying missing channels (color: lab(none 0 0)).\
         \n  ,\
         \n2 | a {b: color.scale(lab(none 0 0), $lightness: 10%, $space: lab)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
