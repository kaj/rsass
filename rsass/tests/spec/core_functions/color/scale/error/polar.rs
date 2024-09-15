//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/error/polar.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("polar")
}

#[test]
#[ignore] // wrong error
fn lch() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(lch(50% 30% 180deg), $hue: 10%)}\n"
        ),
        "Error: $hue: Channel isn\'t scalable.\
         \n  ,\
         \n2 | a {b: color.scale(lch(50% 30% 180deg), $hue: 10%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn legacy() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(white, $hue: 10%)}\n"
        ),
        "Error: $hue: Channel isn\'t scalable.\
         \n  ,\
         \n2 | a {b: color.scale(white, $hue: 10%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn oklch() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(lch(50% 30% 180deg), $hue: 10%)}\n"
        ),
        "Error: $hue: Channel isn\'t scalable.\
         \n  ,\
         \n2 | a {b: color.scale(lch(50% 30% 180deg), $hue: 10%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
