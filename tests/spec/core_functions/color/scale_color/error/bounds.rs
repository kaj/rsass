//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale_color/error/bounds.hrx"

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
            runner().err("a {b: scale-color(red, $alpha: 100.001%)}\n"),
            "Error: $alpha: Expected 100.001% to be within -100% and 100%.\
         \n  ,\
         \n1 | a {b: scale-color(red, $alpha: 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn too_low() {
        assert_eq!(
            runner().err("a {b: scale-color(red, $alpha: -100.001%)}\n"),
            "Error: $alpha: Expected -100.001% to be within -100% and 100%.\
         \n  ,\
         \n1 | a {b: scale-color(red, $alpha: -100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
        runner().err(
            "a {b: scale-color(red, $blackness: 100.001%)}\n"
        ),
        "Error: $blackness: Expected 100.001% to be within -100% and 100%.\
         \n  ,\
         \n1 | a {b: scale-color(red, $blackness: 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // missing error
    fn too_low() {
        assert_eq!(
        runner().err(
            "a {b: scale-color(red, $blackness: -100.001%)}\n"
        ),
        "Error: $blackness: Expected -100.001% to be within -100% and 100%.\
         \n  ,\
         \n1 | a {b: scale-color(red, $blackness: -100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
            runner().err("a {b: scale-color(blue, $blue: 100.001%)}\n"),
            "Error: $blue: Expected 100.001% to be within -100% and 100%.\
         \n  ,\
         \n1 | a {b: scale-color(blue, $blue: 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn too_low() {
        assert_eq!(
            runner().err("a {b: scale-color(blue, $blue: -100.001%)}\n"),
            "Error: $blue: Expected -100.001% to be within -100% and 100%.\
         \n  ,\
         \n1 | a {b: scale-color(blue, $blue: -100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
            runner().err("a {b: scale-color(green, $green: 100.001%)}\n"),
            "Error: $green: Expected 100.001% to be within -100% and 100%.\
         \n  ,\
         \n1 | a {b: scale-color(green, $green: 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn too_low() {
        assert_eq!(
            runner().err("a {b: scale-color(green, $green: -100.001%)}\n"),
            "Error: $green: Expected -100.001% to be within -100% and 100%.\
         \n  ,\
         \n1 | a {b: scale-color(green, $green: -100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
        runner().err(
            "a {b: scale-color(red, $lightness: 100.001%)}\n"
        ),
        "Error: $lightness: Expected 100.001% to be within -100% and 100%.\
         \n  ,\
         \n1 | a {b: scale-color(red, $lightness: 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // missing error
    fn too_low() {
        assert_eq!(
        runner().err(
            "a {b: scale-color(red, $lightness: -100.001%)}\n"
        ),
        "Error: $lightness: Expected -100.001% to be within -100% and 100%.\
         \n  ,\
         \n1 | a {b: scale-color(red, $lightness: -100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
            runner().err("a {b: scale-color(red, $red: 100.001%)}\n"),
            "Error: $red: Expected 100.001% to be within -100% and 100%.\
         \n  ,\
         \n1 | a {b: scale-color(red, $red: 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn too_low() {
        assert_eq!(
            runner().err("a {b: scale-color(red, $red: -100.001%)}\n"),
            "Error: $red: Expected -100.001% to be within -100% and 100%.\
         \n  ,\
         \n1 | a {b: scale-color(red, $red: -100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
        runner().err(
            "a {b: scale-color(red, $saturation: 100.001%)}\n"
        ),
        "Error: $saturation: Expected 100.001% to be within -100% and 100%.\
         \n  ,\
         \n1 | a {b: scale-color(red, $saturation: 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // missing error
    fn too_low() {
        assert_eq!(
        runner().err(
            "a {b: scale-color(red, $saturation: -100.001%)}\n"
        ),
        "Error: $saturation: Expected -100.001% to be within -100% and 100%.\
         \n  ,\
         \n1 | a {b: scale-color(red, $saturation: -100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
        runner().err(
            "a {b: scale-color(red, $whiteness: 100.001%)}\n"
        ),
        "Error: $whiteness: Expected 100.001% to be within -100% and 100%.\
         \n  ,\
         \n1 | a {b: scale-color(red, $whiteness: 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // missing error
    fn too_low() {
        assert_eq!(
        runner().err(
            "a {b: scale-color(red, $whiteness: -100.001%)}\n"
        ),
        "Error: $whiteness: Expected -100.001% to be within -100% and 100%.\
         \n  ,\
         \n1 | a {b: scale-color(red, $whiteness: -100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
}
