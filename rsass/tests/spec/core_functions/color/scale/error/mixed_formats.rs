//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/error/mixed_formats.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mixed_formats")
}

#[test]
#[ignore] // wrong error
fn blue_and_lightness() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(red, $blue: 1%, $lightness: 1%)}\n"
        ),
        "Error: $lightness: Color space rgb doesn\'t have a channel with this name.\
         \n  ,\
         \n2 | a {b: color.scale(red, $blue: 1%, $lightness: 1%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn green_and_saturation() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(red, $green: 1%, $saturation: 1%)}\n"
        ),
        "Error: $saturation: Color space rgb doesn\'t have a channel with this name.\
         \n  ,\
         \n2 | a {b: color.scale(red, $green: 1%, $saturation: 1%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn green_and_whiteness() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(red, $green: 1%, $whiteness: 1%)}\n"
        ),
        "Error: $whiteness: Color space rgb doesn\'t have a channel with this name.\
         \n  ,\
         \n2 | a {b: color.scale(red, $green: 1%, $whiteness: 1%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn lightness_and_whiteness() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(red, $lightness: 1%, $whiteness: 1%)}\n"
        ),
        "Error: $whiteness: Color space hsl doesn\'t have a channel with this name.\
         \n  ,\
         \n2 | a {b: color.scale(red, $lightness: 1%, $whiteness: 1%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn red_and_blackness() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(red, $red: 1%, $blackness: 1%)}\n"
        ),
        "Error: $blackness: Color space rgb doesn\'t have a channel with this name.\
         \n  ,\
         \n2 | a {b: color.scale(red, $red: 1%, $blackness: 1%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn red_and_saturation() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(red, $red: 1%, $saturation: 1%)}\n"
        ),
        "Error: $saturation: Color space rgb doesn\'t have a channel with this name.\
         \n  ,\
         \n2 | a {b: color.scale(red, $red: 1%, $saturation: 1%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn saturation_and_blackness() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(red, $saturation: 1%, $blackness: 1%)}\n"
        ),
        "Error: $blackness: Color space hsl doesn\'t have a channel with this name.\
         \n  ,\
         \n2 | a {b: color.scale(red, $saturation: 1%, $blackness: 1%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
