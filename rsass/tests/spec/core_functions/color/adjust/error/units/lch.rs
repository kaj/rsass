//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust/error/units/lch.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lch")
}

#[test]
#[ignore] // wrong error
fn chroma() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.adjust(lch(50% 30 50deg), $chroma: 20px)}\n"
        ),
        "Error: $chroma: Expected 20px to have unit \"%\" or no units.\
         \n  ,\
         \n2 | a {b: color.adjust(lch(50% 30 50deg), $chroma: 20px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: color.adjust(lch(50% 30 50deg), $hue: 20%)}\n"
        ),
        "Error: $hue: Expected 20% to have an angle unit (deg, grad, rad, turn).\
         \n  ,\
         \n2 | a {b: color.adjust(lch(50% 30 50deg), $hue: 20%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: color.adjust(lch(50% 30 50deg), $lightness: 30px)}\n"
        ),
        "Error: $lightness: Expected 30px to have unit \"%\" or no units.\
         \n  ,\
         \n2 | a {b: color.adjust(lch(50% 30 50deg), $lightness: 30px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
