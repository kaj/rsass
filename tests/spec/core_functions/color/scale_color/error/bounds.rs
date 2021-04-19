//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale_color/error/bounds.hrx"

mod alpha {
    #[test]
    #[ignore] // missing error
    fn too_high() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(red, $alpha: 100.001%)}\
             \n"
            )
            .unwrap_err(),
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
            crate::rsass(
                "a {b: scale-color(red, $alpha: -100.001%)}\
             \n"
            )
            .unwrap_err(),
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
    #[test]
    #[ignore] // missing error
    fn too_high() {
        assert_eq!(
        crate::rsass(
            "a {b: scale-color(red, $blackness: 100.001%)}\
             \n"
        ).unwrap_err(),
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
        crate::rsass(
            "a {b: scale-color(red, $blackness: -100.001%)}\
             \n"
        ).unwrap_err(),
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
    #[test]
    #[ignore] // missing error
    fn too_high() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(blue, $blue: 100.001%)}\
             \n"
            )
            .unwrap_err(),
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
            crate::rsass(
                "a {b: scale-color(blue, $blue: -100.001%)}\
             \n"
            )
            .unwrap_err(),
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
    #[test]
    #[ignore] // missing error
    fn too_high() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(green, $green: 100.001%)}\
             \n"
            )
            .unwrap_err(),
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
            crate::rsass(
                "a {b: scale-color(green, $green: -100.001%)}\
             \n"
            )
            .unwrap_err(),
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
    #[test]
    #[ignore] // missing error
    fn too_high() {
        assert_eq!(
        crate::rsass(
            "a {b: scale-color(red, $lightness: 100.001%)}\
             \n"
        ).unwrap_err(),
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
        crate::rsass(
            "a {b: scale-color(red, $lightness: -100.001%)}\
             \n"
        ).unwrap_err(),
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
    #[test]
    #[ignore] // missing error
    fn too_high() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(red, $red: 100.001%)}\
             \n"
            )
            .unwrap_err(),
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
            crate::rsass(
                "a {b: scale-color(red, $red: -100.001%)}\
             \n"
            )
            .unwrap_err(),
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
    #[test]
    #[ignore] // missing error
    fn too_high() {
        assert_eq!(
        crate::rsass(
            "a {b: scale-color(red, $saturation: 100.001%)}\
             \n"
        ).unwrap_err(),
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
        crate::rsass(
            "a {b: scale-color(red, $saturation: -100.001%)}\
             \n"
        ).unwrap_err(),
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
    #[test]
    #[ignore] // missing error
    fn too_high() {
        assert_eq!(
        crate::rsass(
            "a {b: scale-color(red, $whiteness: 100.001%)}\
             \n"
        ).unwrap_err(),
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
        crate::rsass(
            "a {b: scale-color(red, $whiteness: -100.001%)}\
             \n"
        ).unwrap_err(),
        "Error: $whiteness: Expected -100.001% to be within -100% and 100%.\
         \n  ,\
         \n1 | a {b: scale-color(red, $whiteness: -100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
}
