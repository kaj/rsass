//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust/error/units/xyz.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("xyz")
}

#[test]
#[ignore] // wrong error
fn blue() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.adjust(color(xyz 0.2 0.5 0.7), $z: 0.5px)}\n"
        ),
        "Error: $z: Expected 0.5px to have unit \"%\" or no units.\
         \n  ,\
         \n2 | a {b: color.adjust(color(xyz 0.2 0.5 0.7), $z: 0.5px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: color.adjust(color(xyz 0.2 0.5 0.7), $y: 0.5px)}\n"
        ),
        "Error: $y: Expected 0.5px to have unit \"%\" or no units.\
         \n  ,\
         \n2 | a {b: color.adjust(color(xyz 0.2 0.5 0.7), $y: 0.5px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: color.adjust(color(xyz 0.2 0.5 0.7), $x: 0.5px)}\n"
        ),
        "Error: $x: Expected 0.5px to have unit \"%\" or no units.\
         \n  ,\
         \n2 | a {b: color.adjust(color(xyz 0.2 0.5 0.7), $x: 0.5px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
