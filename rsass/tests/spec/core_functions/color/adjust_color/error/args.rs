//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust_color/error/args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("args")
}

#[test]
fn too_few() {
    assert_eq!(
        runner().err("a {b: adjust-color()}\n"),
        "Error: Missing argument $color.\
         \n  ,--> input.scss\
         \n1 | a {b: adjust-color()}\
         \n  |       ^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function adjust($color, $kwargs...) {\
         \n  |           ========================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
fn too_many() {
    assert_eq!(
        runner().err(
            "a {b: adjust-color(red, 1)}\n"
        ),
        "Error: Only one positional argument is allowed. All other arguments must be passed by name.\
         \n  ,\
         \n1 | a {b: adjust-color(red, 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
fn unknown() {
    assert_eq!(
        runner().err("a {b: adjust-color(red, $ambience: 10%)}\n"),
        "Error: No argument named $ambience.\
         \n  ,\
         \n1 | a {b: adjust-color(red, $ambience: 10%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
