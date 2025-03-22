//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/error/args.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("args")
}

#[test]
fn too_few() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale()}\n"
        ),
        "Error: Missing argument $color.\
         \n  ,--> input.scss\
         \n2 | a {b: color.scale()}\
         \n  |       ^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function scale($color, $kwargs...) {\
         \n  |           ========================= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn too_many() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(red, 1)}\n"
        ),
        "Error: Only one positional argument is allowed. All other arguments must be passed by name.\
         \n  ,\
         \n2 | a {b: color.scale(red, 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn unknown() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(red, $hue: 10%)}\n"
        ),
        "Error: $hue: Channel isn\'t scalable.\
         \n  ,\
         \n2 | a {b: color.scale(red, $hue: 10%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn wrong_name() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale-color(#abcdef, $red: 10%)}\n"
        ),
        "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: color.scale-color(#abcdef, $red: 10%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
