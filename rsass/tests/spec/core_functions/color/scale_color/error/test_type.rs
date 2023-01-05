//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale_color/error/type.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("type")
}

#[test]
fn alpha() {
    assert_eq!(
        runner().err("a {b: scale-color(red, $alpha: c)}\n"),
        "Error: $alpha: c is not a number.\
         \n  ,\
         \n1 | a {b: scale-color(red, $alpha: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
fn blackness() {
    assert_eq!(
        runner().err("a {b: scale-color(red, $blackness: c)}\n"),
        "Error: $blackness: c is not a number.\
         \n  ,\
         \n1 | a {b: scale-color(red, $blackness: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
fn blue() {
    assert_eq!(
        runner().err("a {b: scale-color(red, $blue: c)}\n"),
        "Error: $blue: c is not a number.\
         \n  ,\
         \n1 | a {b: scale-color(red, $blue: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
fn color() {
    assert_eq!(
        runner().err("a {b: scale-color(1)}\n"),
        "Error: $color: 1 is not a color.\
         \n  ,\
         \n1 | a {b: scale-color(1)}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
fn green() {
    assert_eq!(
        runner().err("a {b: scale-color(red, $green: c)}\n"),
        "Error: $green: c is not a number.\
         \n  ,\
         \n1 | a {b: scale-color(red, $green: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
fn lightness() {
    assert_eq!(
        runner().err("a {b: scale-color(red, $lightness: c)}\n"),
        "Error: $lightness: c is not a number.\
         \n  ,\
         \n1 | a {b: scale-color(red, $lightness: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
fn red() {
    assert_eq!(
        runner().err("a {b: scale-color(red, $red: c)}\n"),
        "Error: $red: c is not a number.\
         \n  ,\
         \n1 | a {b: scale-color(red, $red: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
fn saturation() {
    assert_eq!(
        runner().err("a {b: scale-color(red, $saturation: c)}\n"),
        "Error: $saturation: c is not a number.\
         \n  ,\
         \n1 | a {b: scale-color(red, $saturation: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
fn whiteness() {
    assert_eq!(
        runner().err("a {b: scale-color(red, $whiteness: c)}\n"),
        "Error: $whiteness: c is not a number.\
         \n  ,\
         \n1 | a {b: scale-color(red, $whiteness: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
