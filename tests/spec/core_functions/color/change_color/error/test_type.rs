//! Tests auto-converted from "sass-spec/spec/core_functions/color/change_color/error/type.hrx"

#[test]
fn alpha() {
    assert_eq!(
        crate::rsass(
            "a {b: change-color(red, $alpha: c)}\
             \n"
        )
        .unwrap_err(),
        "Error: $alpha: c is not a number.\
         \n  ,\
         \n1 | a {b: change-color(red, $alpha: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
    );
}
#[test]
fn blackness() {
    assert_eq!(
        crate::rsass(
            "a {b: change-color(red, $blackness: c)}\
             \n"
        )
        .unwrap_err(),
        "Error: $blackness: c is not a number.\
         \n  ,\
         \n1 | a {b: change-color(red, $blackness: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
    );
}
#[test]
fn blue() {
    assert_eq!(
        crate::rsass(
            "a {b: change-color(red, $blue: c)}\
             \n"
        )
        .unwrap_err(),
        "Error: $blue: c is not a number.\
         \n  ,\
         \n1 | a {b: change-color(red, $blue: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
    );
}
#[test]
fn color() {
    assert_eq!(
        crate::rsass(
            "a {b: change-color(1)}\
             \n"
        )
        .unwrap_err(),
        "Error: $color: 1 is not a color.\
         \n  ,\
         \n1 | a {b: change-color(1)}\
         \n  |       ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
    );
}
#[test]
fn green() {
    assert_eq!(
        crate::rsass(
            "a {b: change-color(red, $green: c)}\
             \n"
        )
        .unwrap_err(),
        "Error: $green: c is not a number.\
         \n  ,\
         \n1 | a {b: change-color(red, $green: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
    );
}
#[test]
fn hue() {
    assert_eq!(
        crate::rsass(
            "a {b: change-color(red, $hue: c)}\
             \n"
        )
        .unwrap_err(),
        "Error: $hue: c is not a number.\
         \n  ,\
         \n1 | a {b: change-color(red, $hue: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
    );
}
#[test]
fn lightness() {
    assert_eq!(
        crate::rsass(
            "a {b: change-color(red, $lightness: c)}\
             \n"
        )
        .unwrap_err(),
        "Error: $lightness: c is not a number.\
         \n  ,\
         \n1 | a {b: change-color(red, $lightness: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
    );
}
#[test]
fn red() {
    assert_eq!(
        crate::rsass(
            "a {b: change-color(red, $red: c)}\
             \n"
        )
        .unwrap_err(),
        "Error: $red: c is not a number.\
         \n  ,\
         \n1 | a {b: change-color(red, $red: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
    );
}
#[test]
fn saturation() {
    assert_eq!(
        crate::rsass(
            "a {b: change-color(red, $saturation: c)}\
             \n"
        )
        .unwrap_err(),
        "Error: $saturation: c is not a number.\
         \n  ,\
         \n1 | a {b: change-color(red, $saturation: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
    );
}
#[test]
#[ignore] // wrong error
fn whiteness() {
    assert_eq!(
        crate::rsass(
            "a {b: change-color(red, $whiteness: c)}\
             \n"
        )
        .unwrap_err(),
        "Error: $whiteness: c is not a number.\
         \n  ,\
         \n1 | a {b: change-color(red, $whiteness: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n\
         \n",
    );
}
