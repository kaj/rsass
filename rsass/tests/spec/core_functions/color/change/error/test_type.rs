//! Tests auto-converted from "sass-spec/spec/core_functions/color/change/error/type.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("type")
}

#[test]
#[ignore] // wrong error
fn alpha() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.change(red, $alpha: c)}\n"
        ),
        "Error: $alpha: c is not a number or unquoted \"none\".\
         \n  ,\
         \n2 | a {b: color.change(red, $alpha: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn blackness() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.change(red, $blackness: c)}\n"
        ),
        "Error: $blackness: c is not a number or unquoted \"none\".\
         \n  ,\
         \n2 | a {b: color.change(red, $blackness: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn blue() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.change(red, $blue: c)}\n"
        ),
        "Error: $blue: c is not a number or unquoted \"none\".\
         \n  ,\
         \n2 | a {b: color.change(red, $blue: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn color() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.change(1)}\n"
        ),
        "Error: $color: 1 is not a color.\
         \n  ,\
         \n2 | a {b: color.change(1)}\
         \n  |       ^^^^^^^^^^^^^^^\
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
             \na {b: color.change(red, $green: c)}\n"
        ),
        "Error: $green: c is not a number or unquoted \"none\".\
         \n  ,\
         \n2 | a {b: color.change(red, $green: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: color.change(red, $hue: c)}\n"
        ),
        "Error: $hue: c is not a number or unquoted \"none\".\
         \n  ,\
         \n2 | a {b: color.change(red, $hue: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: color.change(red, $lightness: c)}\n"
        ),
        "Error: $lightness: c is not a number or unquoted \"none\".\
         \n  ,\
         \n2 | a {b: color.change(red, $lightness: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn quoted_none() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.change(red, $alpha: \"none\")}\n"
        ),
        "Error: $alpha: \"none\" is not a number or unquoted \"none\".\
         \n  ,\
         \n2 | a {b: color.change(red, $alpha: \"none\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: color.change(red, $red: c)}\n"
        ),
        "Error: $red: c is not a number or unquoted \"none\".\
         \n  ,\
         \n2 | a {b: color.change(red, $red: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn saturation() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.change(red, $saturation: c)}\n"
        ),
        "Error: $saturation: c is not a number or unquoted \"none\".\
         \n  ,\
         \n2 | a {b: color.change(red, $saturation: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: color.change(red, $space: 1)}\n"
        ),
        "Error: $space: 1 is not a string.\
         \n  ,\
         \n2 | a {b: color.change(red, $space: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn whiteness() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.change(red, $whiteness: c)}\n"
        ),
        "Error: $whiteness: c is not a number or unquoted \"none\".\
         \n  ,\
         \n2 | a {b: color.change(red, $whiteness: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
