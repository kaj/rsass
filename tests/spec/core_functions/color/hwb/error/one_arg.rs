//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/error/one_arg.hrx"

mod alpha {
    #[test]
    #[ignore] // missing error
    fn unit() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
             \na {b: color.hwb(0 0 0 / 0.5px)}\
             \n"
            )
            .unwrap_err(),
            "Error: $whiteness: Expected 0 to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.hwb(0 0 0 / 0.5px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn var() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
             \na {b: color.hwb(0 0 0 / var(--c))}\
             \n"
            )
            .unwrap_err(),
            "Error: Expected numeric channels, got \"hwb(0 0 0/var(--c))\".\
         \n  ,\
         \n2 | a {b: color.hwb(0 0 0 / var(--c))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
mod blackness {
    #[test]
    #[ignore] // wrong error
    fn too_high() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
             \na {b: color.hwb(0 30% 101%)}\
             \n"
            )
            .unwrap_err(),
            "Error: $blackness: Expected 101% to be within 0% and 100%.\
         \n  ,\
         \n2 | a {b: color.hwb(0 30% 101%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn too_low() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
             \na {b: color.hwb(0 30% -1%)}\
             \n"
            )
            .unwrap_err(),
            "Error: $blackness: Expected -1% to be within 0% and 100%.\
         \n  ,\
         \n2 | a {b: color.hwb(0 30% -1%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
             \na {b: color.hwb(0 30% \"foo\")}\
             \n"
            )
            .unwrap_err(),
            "Error: $blackness: \"foo\" is not a number.\
         \n  ,\
         \n2 | a {b: color.hwb(0 30% \"foo\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    mod unit {
        #[test]
        #[ignore] // missing error
        fn none() {
            assert_eq!(
                crate::rsass(
                    "@use \'sass:color\';\
             \na {b: color.hwb(0 30% 40)}\
             \n"
                )
                .unwrap_err(),
                "Error: $blackness: Expected 40 to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.hwb(0 30% 40)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn wrong() {
            assert_eq!(
                crate::rsass(
                    "@use \'sass:color\';\
             \na {b: color.hwb(0 30% 40px)}\
             \n"
                )
                .unwrap_err(),
                "Error: $blackness: Expected 40px to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.hwb(0 30% 40px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
    mod var {
        #[test]
        fn alpha() {
            assert_eq!(
        crate::rsass(
            "@use \'sass:color\';\
             \na {b: color.hwb(0 30% var(--c) / 0.5px)}\
             \n"
        ).unwrap_err(),
        "Error: Expected numeric channels, got \"hwb(0 30% var(--c)/0.5px)\".\
         \n  ,\
         \n2 | a {b: color.hwb(0 30% var(--c) / 0.5px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
        #[test]
        fn no_alpha() {
            assert_eq!(
                crate::rsass(
                    "@use \'sass:color\';\
             \na {b: color.hwb(0 30% var(--c))}\
             \n"
                )
                .unwrap_err(),
                "Error: $blackness: var(--c) is not a number.\
         \n  ,\
         \n2 | a {b: color.hwb(0 30% var(--c))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
}
mod hue {
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
             \na {b: color.hwb(\"foo\" 30% 40%)}\
             \n"
            )
            .unwrap_err(),
            "Error: $hue: \"foo\" is not a number.\
         \n  ,\
         \n2 | a {b: color.hwb(\"foo\" 30% 40%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn var() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
             \na {b: color.hwb(var(--c) 30% 40%)}\
             \n"
            )
            .unwrap_err(),
            "Error: $hue: var(--c) is not a number.\
         \n  ,\
         \n2 | a {b: color.hwb(var(--c) 30% 40%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
mod list {
    #[test]
    #[ignore] // missing error
    fn bracketed() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
             \na {b: color.hwb([0 30% 40%])}\
             \n"
            )
            .unwrap_err(),
            "Error: $channels must be an unbracketed list.\
         \n  ,\
         \n2 | a {b: color.hwb([0 30% 40%])}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn comma_separated() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
             \na {b: color.hwb((0, 30%, 40%))}\
             \n"
            )
            .unwrap_err(),
            "Error: $channels must be a space-separated list.\
         \n  ,\
         \n2 | a {b: color.hwb((0, 30%, 40%))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn empty() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
             \na {b: color.hwb(())}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing element $hue.\
         \n  ,\
         \n2 | a {b: color.hwb(())}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn four_elements() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
             \na {b: color.hwb(0 30% 40% 0.4)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 3 elements allowed, but 4 were passed.\
         \n  ,\
         \n2 | a {b: color.hwb(0 30% 40% 0.4)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn one_element() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
             \na {b: color.hwb(0)}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing element $whiteness.\
         \n  ,\
         \n2 | a {b: color.hwb(0)}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn two_elements() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
             \na {b: color.hwb(0 30%)}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing element $blackness.\
         \n  ,\
         \n2 | a {b: color.hwb(0 30%)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn quoted_var_slash() {
    assert_eq!(
        crate::rsass(
            "@use \'sass:color\';\
             \na {b: color.hwb(0 30% \"var(--foo) / 0.4\")}\
             \n"
        )
        .unwrap_err(),
        "Error: $blackness: \"var(--foo) / 0.4\" is not a number.\
         \n  ,\
         \n2 | a {b: color.hwb(0 30% \"var(--foo) / 0.4\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
mod whiteness {
    #[test]
    #[ignore] // wrong error
    fn too_high() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
             \na {b: color.hwb(0 101% 40%)}\
             \n"
            )
            .unwrap_err(),
            "Error: $whiteness: Expected 101% to be within 0% and 100%.\
         \n  ,\
         \n2 | a {b: color.hwb(0 101% 40%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn too_low() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
             \na {b: color.hwb(0 -1% 40%)}\
             \n"
            )
            .unwrap_err(),
            "Error: $whiteness: Expected -1% to be within 0% and 100%.\
         \n  ,\
         \n2 | a {b: color.hwb(0 -1% 40%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
             \na {b: color.hwb(0 \"foo\" 40%)}\
             \n"
            )
            .unwrap_err(),
            "Error: $whiteness: \"foo\" is not a number.\
         \n  ,\
         \n2 | a {b: color.hwb(0 \"foo\" 40%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    mod unit {
        #[test]
        #[ignore] // missing error
        fn none() {
            assert_eq!(
                crate::rsass(
                    "@use \'sass:color\';\
             \na {b: color.hwb(0 30 40%)}\
             \n"
                )
                .unwrap_err(),
                "Error: $whiteness: Expected 30 to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.hwb(0 30 40%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn wrong() {
            assert_eq!(
                crate::rsass(
                    "@use \'sass:color\';\
             \na {b: color.hwb(0 30px 40%)}\
             \n"
                )
                .unwrap_err(),
                "Error: $whiteness: Expected 30px to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.hwb(0 30px 40%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
    #[test]
    fn var() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
             \na {b: color.hwb(0 var(--c) 40%)}\
             \n"
            )
            .unwrap_err(),
            "Error: $whiteness: var(--c) is not a number.\
         \n  ,\
         \n2 | a {b: color.hwb(0 var(--c) 40%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
