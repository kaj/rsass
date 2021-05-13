//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust_color/error/mixed_formats.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // missing error
fn blue_and_lightness() {
    assert_eq!(
        runner().err("a {b: adjust-color(red, $blue: 1, $lightness: 1%)}\n"),
        "Error: RGB parameters may not be passed along with HSL parameters.\
         \n  ,\
         \n1 | a {b: adjust-color(red, $blue: 1, $lightness: 1%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn green_and_saturation() {
    assert_eq!(
        runner()
            .err("a {b: adjust-color(red, $green: 1, $saturation: 1%)}\n"),
        "Error: RGB parameters may not be passed along with HSL parameters.\
         \n  ,\
         \n1 | a {b: adjust-color(red, $green: 1, $saturation: 1%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn green_and_whiteness() {
    assert_eq!(
        runner().err("a {b: adjust-color(red, $green: 1, $whiteness: 1%)}\n"),
        "Error: RGB parameters may not be passed along with HWB parameters.\
         \n  ,\
         \n1 | a {b: adjust-color(red, $green: 1, $whiteness: 1%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn lightness_and_whiteness() {
    assert_eq!(
        runner().err(
            "a {b: adjust-color(red, $lightness: 1%, $whiteness: 1%)}\n"
        ),
        "Error: HSL parameters may not be passed along with HWB parameters.\
         \n  ,\
         \n1 | a {b: adjust-color(red, $lightness: 1%, $whiteness: 1%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn red_and_blackness() {
    assert_eq!(
        runner().err("a {b: adjust-color(red, $red: 1, $blackness: 1%)}\n"),
        "Error: RGB parameters may not be passed along with HWB parameters.\
         \n  ,\
         \n1 | a {b: adjust-color(red, $red: 1, $blackness: 1%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn red_and_hue() {
    assert_eq!(
        runner().err("a {b: adjust-color(red, $red: 1, $hue: 1)}\n"),
        "Error: RGB parameters may not be passed along with HSL parameters.\
         \n  ,\
         \n1 | a {b: adjust-color(red, $red: 1, $hue: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn saturation_and_blackness() {
    assert_eq!(
        runner().err(
            "a {b: adjust-color(red, $saturation: 1%, $blackness: 1%)}\n"
        ),
        "Error: HSL parameters may not be passed along with HWB parameters.\
         \n  ,\
         \n1 | a {b: adjust-color(red, $saturation: 1%, $blackness: 1%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
