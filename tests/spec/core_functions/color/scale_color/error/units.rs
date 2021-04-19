//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale_color/error/units.hrx"

mod none {
    #[test]
    #[ignore] // missing error
    fn alpha() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(red, $alpha: 1)}\
             \n"
            )
            .unwrap_err(),
            "Error: $alpha: Expected 1 to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(red, $alpha: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn blackness() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(black, $blackness: 1)}\
             \n"
            )
            .unwrap_err(),
            "Error: $blackness: Expected 1 to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(black, $blackness: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn blue() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(blue, $blue: 1)}\
             \n"
            )
            .unwrap_err(),
            "Error: $blue: Expected 1 to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(blue, $blue: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn green() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(green, $green: 1)}\
             \n"
            )
            .unwrap_err(),
            "Error: $green: Expected 1 to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(green, $green: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn lightness() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(red, $lightness: 1)}\
             \n"
            )
            .unwrap_err(),
            "Error: $lightness: Expected 1 to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(red, $lightness: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn red() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(red, $red: 1px)}\
             \n"
            )
            .unwrap_err(),
            "Error: $red: Expected 1px to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(red, $red: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn saturation() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(red, $saturation: 1)}\
             \n"
            )
            .unwrap_err(),
            "Error: $saturation: Expected 1 to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(red, $saturation: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn whiteness() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(white, $whiteness: 1)}\
             \n"
            )
            .unwrap_err(),
            "Error: $whiteness: Expected 1 to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(white, $whiteness: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n\
         \n",
        );
    }
}
mod wrong {
    #[test]
    #[ignore] // missing error
    fn alpha() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(red, $alpha: 1px)}\
             \n"
            )
            .unwrap_err(),
            "Error: $alpha: Expected 1px to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(red, $alpha: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn blackness() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(black, $blackness: 1px)}\
             \n"
            )
            .unwrap_err(),
            "Error: $blackness: Expected 1px to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(black, $blackness: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn blue() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(blue, $blue: 1px)}\
             \n"
            )
            .unwrap_err(),
            "Error: $blue: Expected 1px to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(blue, $blue: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn green() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(green, $green: 1px)}\
             \n"
            )
            .unwrap_err(),
            "Error: $green: Expected 1px to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(green, $green: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn lightness() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(red, $lightness: 1px)}\
             \n"
            )
            .unwrap_err(),
            "Error: $lightness: Expected 1px to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(red, $lightness: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn red() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(red, $red: 1px)}\
             \n"
            )
            .unwrap_err(),
            "Error: $red: Expected 1px to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(red, $red: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn saturation() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(red, $saturation: 1px)}\
             \n"
            )
            .unwrap_err(),
            "Error: $saturation: Expected 1px to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(red, $saturation: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn whiteness() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(white, $whiteness: 1px)}\
             \n"
            )
            .unwrap_err(),
            "Error: $whiteness: Expected 1px to have unit \"%\".\
         \n  ,\
         \n1 | a {b: scale-color(white, $whiteness: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
}
