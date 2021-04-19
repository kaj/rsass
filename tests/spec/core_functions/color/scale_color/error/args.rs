//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale_color/error/args.hrx"

#[test]
#[ignore] // wrong error
fn too_few() {
    assert_eq!(
        crate::rsass(
            "a {b: scale-color()}\
             \n"
        )
        .unwrap_err(),
        "Error: Missing argument $color.\
         \n  ,--> input.scss\
         \n1 | a {b: scale-color()}\
         \n  |       ^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function scale($color, $kwargs...) {\
         \n  |           ========================= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn too_many() {
    assert_eq!(
        crate::rsass(
            "a {b: scale-color(red, 1)}\
             \n"
        ).unwrap_err(),
        "Error: Only one positional argument is allowed. All other arguments must be passed by name.\
         \n  ,\
         \n1 | a {b: scale-color(red, 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn unknown() {
    assert_eq!(
        crate::rsass(
            "a {b: scale-color(red, $hue: 10%)}\
             \n"
        )
        .unwrap_err(),
        "Error: No argument named $hue.\
         \n  ,\
         \n1 | a {b: scale-color(red, $hue: 10%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
