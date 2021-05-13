//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale_color/error/units.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod none {
    #[allow(unused)]
    use super::runner;
    #[test]
    #[ignore] // missing error
    fn alpha() {
        assert_eq!(
            runner().err("a {b: scale-color(red, $alpha: 1)}\n"),
            "Error: $alpha: Expected 1 to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(red, $alpha: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn blackness() {
        assert_eq!(
            runner().err("a {b: scale-color(black, $blackness: 1)}\n"),
            "Error: $blackness: Expected 1 to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(black, $blackness: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn blue() {
        assert_eq!(
            runner().err("a {b: scale-color(blue, $blue: 1)}\n"),
            "Error: $blue: Expected 1 to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(blue, $blue: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn green() {
        assert_eq!(
            runner().err("a {b: scale-color(green, $green: 1)}\n"),
            "Error: $green: Expected 1 to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(green, $green: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn lightness() {
        assert_eq!(
            runner().err("a {b: scale-color(red, $lightness: 1)}\n"),
            "Error: $lightness: Expected 1 to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(red, $lightness: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn red() {
        assert_eq!(
            runner().err("a {b: scale-color(red, $red: 1px)}\n"),
            "Error: $red: Expected 1px to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(red, $red: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn saturation() {
        assert_eq!(
            runner().err("a {b: scale-color(red, $saturation: 1)}\n"),
            "Error: $saturation: Expected 1 to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(red, $saturation: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn whiteness() {
        assert_eq!(
            runner().err("a {b: scale-color(white, $whiteness: 1)}\n"),
            "Error: $whiteness: Expected 1 to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(white, $whiteness: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
mod wrong {
    #[allow(unused)]
    use super::runner;
    #[test]
    #[ignore] // missing error
    fn alpha() {
        assert_eq!(
            runner().err("a {b: scale-color(red, $alpha: 1px)}\n"),
            "Error: $alpha: Expected 1px to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(red, $alpha: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn blackness() {
        assert_eq!(
            runner().err("a {b: scale-color(black, $blackness: 1px)}\n"),
            "Error: $blackness: Expected 1px to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(black, $blackness: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn blue() {
        assert_eq!(
            runner().err("a {b: scale-color(blue, $blue: 1px)}\n"),
            "Error: $blue: Expected 1px to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(blue, $blue: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn green() {
        assert_eq!(
            runner().err("a {b: scale-color(green, $green: 1px)}\n"),
            "Error: $green: Expected 1px to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(green, $green: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn lightness() {
        assert_eq!(
            runner().err("a {b: scale-color(red, $lightness: 1px)}\n"),
            "Error: $lightness: Expected 1px to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(red, $lightness: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn red() {
        assert_eq!(
            runner().err("a {b: scale-color(red, $red: 1px)}\n"),
            "Error: $red: Expected 1px to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(red, $red: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn saturation() {
        assert_eq!(
            runner().err("a {b: scale-color(red, $saturation: 1px)}\n"),
            "Error: $saturation: Expected 1px to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(red, $saturation: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn whiteness() {
        assert_eq!(
            runner().err("a {b: scale-color(white, $whiteness: 1px)}\n"),
            "Error: $whiteness: Expected 1px to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(white, $whiteness: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
