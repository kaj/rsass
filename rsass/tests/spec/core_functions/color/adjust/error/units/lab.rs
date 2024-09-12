//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust/error/units/lab.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lab")
}

#[test]
#[ignore] // wrong error
fn a() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.adjust(lab(50% 30 -50), $a: 20px)}\n"
        ),
        "Error: $a: Expected 20px to have unit \"%\" or no units.\
         \n  ,\
         \n2 | a {b: color.adjust(lab(50% 30 -50), $a: 20px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn b() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.adjust(lab(50% 30 -50), $b: 20px)}\n"
        ),
        "Error: $b: Expected 20px to have unit \"%\" or no units.\
         \n  ,\
         \n2 | a {b: color.adjust(lab(50% 30 -50), $b: 20px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: color.adjust(lab(50% 30 -50), $lightness: 30px)}\n"
        ),
        "Error: $lightness: Expected 30px to have unit \"%\" or no units.\
         \n  ,\
         \n2 | a {b: color.adjust(lab(50% 30 -50), $lightness: 30px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
