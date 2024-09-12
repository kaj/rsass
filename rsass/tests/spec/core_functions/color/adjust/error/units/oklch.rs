//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust/error/units/oklch.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("oklch")
}

#[test]
#[ignore] // wrong error
fn chroma() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.adjust(oklch(50% 0.3 50deg), $chroma: 0.2px)}\n"
        ),
        "Error: $chroma: Expected 0.2px to have unit \"%\" or no units.\
         \n  ,\
         \n2 | a {b: color.adjust(oklch(50% 0.3 50deg), $chroma: 0.2px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn hue() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.adjust(oklch(50% 0.3 50deg), $hue: 20%)}\n"
        ),
        "Error: $hue: Expected 20% to have an angle unit (deg, grad, rad, turn).\
         \n  ,\
         \n2 | a {b: color.adjust(oklch(50% 0.3 50deg), $hue: 20%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn lightness() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.adjust(oklch(50% 0.3 50deg), $lightness: 30px)}\n"
        ),
        "Error: $lightness: Expected 30px to have unit \"%\" or no units.\
         \n  ,\
         \n2 | a {b: color.adjust(oklch(50% 0.3 50deg), $lightness: 30px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
