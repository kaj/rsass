//! Tests auto-converted from "sass-spec/spec/core_functions/color/change_color/error/bounds.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn too_high() {
        assert_eq!(
            runner().err("a {b: change-color(red, $alpha: 1.001)}\n"),
            "Error: $alpha: Expected 1.001 to be within 0 and 1.\
         \n  ,\
         \n1 | a {b: change-color(red, $alpha: 1.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn too_low() {
        assert_eq!(
            runner().err("a {b: change-color(red, $alpha: -0.001)}\n"),
            "Error: $alpha: Expected -0.001 to be within 0 and 1.\
         \n  ,\
         \n1 | a {b: change-color(red, $alpha: -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
mod blackness {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn too_high() {
        assert_eq!(
            runner().err("a {b: change-color(red, $blackness: 100.001%)}\n"),
            "Error: $blackness: Expected 100.001% to be within 0% and 100%.\
         \n  ,\
         \n1 | a {b: change-color(red, $blackness: 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn too_low() {
        assert_eq!(
            runner().err("a {b: change-color(red, $blackness: -0.001%)}\n"),
            "Error: $blackness: Expected -0.001% to be within 0% and 100%.\
         \n  ,\
         \n1 | a {b: change-color(red, $blackness: -0.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
mod blue {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn too_high() {
        assert_eq!(
            runner().err("a {b: change-color(blue, $blue: 256)}\n"),
            "Error: $blue: Expected 256 to be within 0 and 255.\
         \n  ,\
         \n1 | a {b: change-color(blue, $blue: 256)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn too_low() {
        assert_eq!(
            runner().err("a {b: change-color(blue, $blue: -1)}\n"),
            "Error: $blue: Expected -1 to be within 0 and 255.\
         \n  ,\
         \n1 | a {b: change-color(blue, $blue: -1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
mod green {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn too_high() {
        assert_eq!(
            runner().err("a {b: change-color(green, $green: 256)}\n"),
            "Error: $green: Expected 256 to be within 0 and 255.\
         \n  ,\
         \n1 | a {b: change-color(green, $green: 256)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn too_low() {
        assert_eq!(
            runner().err("a {b: change-color(green, $green: -1)}\n"),
            "Error: $green: Expected -1 to be within 0 and 255.\
         \n  ,\
         \n1 | a {b: change-color(green, $green: -1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
mod lightness {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn too_high() {
        assert_eq!(
            runner().err("a {b: change-color(red, $lightness: 100.001%)}\n"),
            "Error: $lightness: Expected 100.001% to be within 0% and 100%.\
         \n  ,\
         \n1 | a {b: change-color(red, $lightness: 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn too_low() {
        assert_eq!(
            runner().err("a {b: change-color(red, $lightness: -0.001%)}\n"),
            "Error: $lightness: Expected -0.001% to be within 0% and 100%.\
         \n  ,\
         \n1 | a {b: change-color(red, $lightness: -0.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
mod red {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn too_high() {
        assert_eq!(
            runner().err("a {b: change-color(red, $red: 256)}\n"),
            "Error: $red: Expected 256 to be within 0 and 255.\
         \n  ,\
         \n1 | a {b: change-color(red, $red: 256)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn too_low() {
        assert_eq!(
            runner().err("a {b: change-color(red, $red: -1)}\n"),
            "Error: $red: Expected -1 to be within 0 and 255.\
         \n  ,\
         \n1 | a {b: change-color(red, $red: -1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
mod saturation {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn too_high() {
        assert_eq!(
            runner().err("a {b: change-color(red, $saturation: 100.001%)}\n"),
            "Error: $saturation: Expected 100.001% to be within 0% and 100%.\
         \n  ,\
         \n1 | a {b: change-color(red, $saturation: 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn too_low() {
        assert_eq!(
            runner().err("a {b: change-color(red, $saturation: -0.001%)}\n"),
            "Error: $saturation: Expected -0.001% to be within 0% and 100%.\
         \n  ,\
         \n1 | a {b: change-color(red, $saturation: -0.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
mod whiteness {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn too_high() {
        assert_eq!(
            runner().err("a {b: change-color(red, $whiteness: 100.001%)}\n"),
            "Error: $whiteness: Expected 100.001% to be within 0% and 100%.\
         \n  ,\
         \n1 | a {b: change-color(red, $whiteness: 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn too_low() {
        assert_eq!(
            runner().err("a {b: change-color(red, $whiteness: -0.001%)}\n"),
            "Error: $whiteness: Expected -0.001% to be within 0% and 100%.\
         \n  ,\
         \n1 | a {b: change-color(red, $whiteness: -0.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
