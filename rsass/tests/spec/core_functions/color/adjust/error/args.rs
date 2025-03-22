//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust/error/args.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("args")
}

#[test]
fn too_few() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.adjust()}\n"
        ),
        "Error: Missing argument $color.\
         \n  ,--> input.scss\
         \n2 | a {b: color.adjust()}\
         \n  |       ^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function adjust($color, $kwargs...) {\
         \n  |           ========================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn too_many() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.adjust(red, 1)}\n"
        ),
        "Error: Only one positional argument is allowed. All other arguments must be passed by name.\
         \n  ,\
         \n2 | a {b: color.adjust(red, 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn unknown() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.adjust(red, $ambience: 10%)}\n"
        ),
        "Error: $ambience: Color space rgb doesn\'t have a channel with this name.\
         \n  ,\
         \n2 | a {b: color.adjust(red, $ambience: 10%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
