//! Tests auto-converted from "sass-spec/spec/core_functions/color/change/error/mixed_formats.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mixed_formats")
}

#[test]
fn blue_and_lightness() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.change(red, $blue: 1, $lightness: 1%)}\n"
        ),
        "Error: $lightness: Color space rgb doesn\'t have a channel with this name.\
         \n  ,\
         \n2 | a {b: color.change(red, $blue: 1, $lightness: 1%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn green_and_saturation() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.change(red, $green: 1, $saturation: 1%)}\n"
        ),
        "Error: $saturation: Color space rgb doesn\'t have a channel with this name.\
         \n  ,\
         \n2 | a {b: color.change(red, $green: 1, $saturation: 1%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn green_and_whiteness() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.change(red, $green: 1, $whiteness: 1%)}\n"
        ),
        "Error: $whiteness: Color space rgb doesn\'t have a channel with this name.\
         \n  ,\
         \n2 | a {b: color.change(red, $green: 1, $whiteness: 1%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn lightness_and_whiteness() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.change(red, $lightness: 1%, $whiteness: 1%)}\n"
        ),
        "Error: $whiteness: Color space hsl doesn\'t have a channel with this name.\
         \n  ,\
         \n2 | a {b: color.change(red, $lightness: 1%, $whiteness: 1%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn red_and_blackness() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.change(red, $red: 1, $blackness: 1%)}\n"
        ),
        "Error: $blackness: Color space rgb doesn\'t have a channel with this name.\
         \n  ,\
         \n2 | a {b: color.change(red, $red: 1, $blackness: 1%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn red_and_hue() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.change(red, $red: 1, $hue: 1)}\n"
        ),
        "Error: $hue: Color space rgb doesn\'t have a channel with this name.\
         \n  ,\
         \n2 | a {b: color.change(red, $red: 1, $hue: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn saturation_and_blackness() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.change(red, $saturation: 1%, $blackness: 1%)}\n"
        ),
        "Error: $blackness: Color space hsl doesn\'t have a channel with this name.\
         \n  ,\
         \n2 | a {b: color.change(red, $saturation: 1%, $blackness: 1%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
