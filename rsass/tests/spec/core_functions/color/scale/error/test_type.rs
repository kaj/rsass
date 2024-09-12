//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/error/type.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("type")
}

#[test]
fn alpha() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(red, $alpha: c)}\n"
        ),
        "Error: $alpha: c is not a number.\
         \n  ,\
         \n2 | a {b: color.scale(red, $alpha: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn blackness() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(red, $blackness: c)}\n"
        ),
        "Error: $blackness: c is not a number.\
         \n  ,\
         \n2 | a {b: color.scale(red, $blackness: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn blue() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(red, $blue: c)}\n"
        ),
        "Error: $blue: c is not a number.\
         \n  ,\
         \n2 | a {b: color.scale(red, $blue: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn color() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(1)}\n"
        ),
        "Error: $color: 1 is not a color.\
         \n  ,\
         \n2 | a {b: color.scale(1)}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn green() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(red, $green: c)}\n"
        ),
        "Error: $green: c is not a number.\
         \n  ,\
         \n2 | a {b: color.scale(red, $green: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn lightness() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(red, $lightness: c)}\n"
        ),
        "Error: $lightness: c is not a number.\
         \n  ,\
         \n2 | a {b: color.scale(red, $lightness: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn none() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(red, $alpha: none)}\n"
        ),
        "Error: $alpha: none is not a number.\
         \n  ,\
         \n2 | a {b: color.scale(red, $alpha: none)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn red() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(red, $red: c)}\n"
        ),
        "Error: $red: c is not a number.\
         \n  ,\
         \n2 | a {b: color.scale(red, $red: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn saturation() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(red, $saturation: c)}\n"
        ),
        "Error: $saturation: c is not a number.\
         \n  ,\
         \n2 | a {b: color.scale(red, $saturation: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn space() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(red, $space: 1)}\n"
        ),
        "Error: $space: 1 is not a string.\
         \n  ,\
         \n2 | a {b: color.scale(red, $space: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn whiteness() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(red, $whiteness: c)}\n"
        ),
        "Error: $whiteness: c is not a number.\
         \n  ,\
         \n2 | a {b: color.scale(red, $whiteness: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
