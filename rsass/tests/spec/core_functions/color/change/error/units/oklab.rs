//! Tests auto-converted from "sass-spec/spec/core_functions/color/change/error/units/oklab.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("oklab")
}

#[test]
#[ignore] // wrong error
fn a() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.change(oklab(50% 0.2 -0.3), $a: 0.2px)}\n"
        ),
        "Error: $a: Expected 0.2px to have unit \"%\" or no units.\
         \n  ,\
         \n2 | a {b: color.change(oklab(50% 0.2 -0.3), $a: 0.2px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: color.change(oklab(50% 0.2 -0.3), $b: 0.2px)}\n"
        ),
        "Error: $b: Expected 0.2px to have unit \"%\" or no units.\
         \n  ,\
         \n2 | a {b: color.change(oklab(50% 0.2 -0.3), $b: 0.2px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: color.change(oklab(50% 0.2 -0.3), $lightness: 0.3px)}\n"
        ),
        "Error: $lightness: Expected 0.3px to have unit \"%\" or no units.\
         \n  ,\
         \n2 | a {b: color.change(oklab(50% 0.2 -0.3), $lightness: 0.3px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
