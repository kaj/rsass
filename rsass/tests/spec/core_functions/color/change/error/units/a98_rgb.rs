//! Tests auto-converted from "sass-spec/spec/core_functions/color/change/error/units/a98_rgb.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("a98_rgb")
}

#[test]
#[ignore] // wrong error
fn blue() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.change(color(a98-rgb 0.2 0.5 0.7), $blue: 0.5px)}\n"
        ),
        "Error: $blue: Expected 0.5px to have unit \"%\" or no units.\
         \n  ,\
         \n2 | a {b: color.change(color(a98-rgb 0.2 0.5 0.7), $blue: 0.5px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn green() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.change(color(a98-rgb 0.2 0.5 0.7), $green: 0.5px)}\n"
        ),
        "Error: $green: Expected 0.5px to have unit \"%\" or no units.\
         \n  ,\
         \n2 | a {b: color.change(color(a98-rgb 0.2 0.5 0.7), $green: 0.5px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn red() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.change(color(a98-rgb 0.2 0.5 0.7), $red: 0.5px)}\n"
        ),
        "Error: $red: Expected 0.5px to have unit \"%\" or no units.\
         \n  ,\
         \n2 | a {b: color.change(color(a98-rgb 0.2 0.5 0.7), $red: 0.5px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
