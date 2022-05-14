//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/error/one_arg.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("one_arg")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn unit() {
        assert_eq!(
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.hwb(0 0 0 / 0.5px)}\n"
            ),
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
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.hwb(0 0 0 / var(--c))}\n"
            ),
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
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_high() {
        assert_eq!(
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.hwb(0 30% 101%)}\n"
            ),
            "Error: $blackness: Expected 101% to be within 0% and 100%.\
         \n  ,\
         \n2 | a {b: color.hwb(0 30% 101%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_low() {
        assert_eq!(
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.hwb(0 30% -1%)}\n"
            ),
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
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.hwb(0 30% \"foo\")}\n"
            ),
            "Error: $blackness: \"foo\" is not a number.\
         \n  ,\
         \n2 | a {b: color.hwb(0 30% \"foo\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    mod unit {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn none() {
            assert_eq!(
                runner().err(
                    "@use \'sass:color\';\
             \na {b: color.hwb(0 30% 40)}\n"
                ),
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
                runner().err(
                    "@use \'sass:color\';\
             \na {b: color.hwb(0 30% 40px)}\n"
                ),
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
        #[allow(unused)]
        use super::runner;

        #[test]
        fn alpha() {
            assert_eq!(
        runner().err(
            "@use \'sass:color\';\
             \na {b: color.hwb(0 30% var(--c) / 0.5px)}\n"
        ),
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
                runner().err(
                    "@use \'sass:color\';\
             \na {b: color.hwb(0 30% var(--c))}\n"
                ),
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
    #[allow(unused)]
    use super::runner;

    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.hwb(\"foo\" 30% 40%)}\n"
            ),
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
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.hwb(var(--c) 30% 40%)}\n"
            ),
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
    #[allow(unused)]
    use super::runner;

    #[test]
    fn bracketed() {
        assert_eq!(
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.hwb([0 30% 40%])}\n"
            ),
            "Error: $channels must be an unbracketed list.\
         \n  ,\
         \n2 | a {b: color.hwb([0 30% 40%])}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn comma_separated() {
        assert_eq!(
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.hwb((0, 30%, 40%))}\n"
            ),
            "Error: $channels must be a space-separated list.\
         \n  ,\
         \n2 | a {b: color.hwb((0, 30%, 40%))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn empty() {
        assert_eq!(
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.hwb(())}\n"
            ),
            "Error: Missing element $hue.\
         \n  ,\
         \n2 | a {b: color.hwb(())}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn four_elements() {
        assert_eq!(
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.hwb(0 30% 40% 0.4)}\n"
            ),
            "Error: Only 3 elements allowed, but 4 were passed.\
         \n  ,\
         \n2 | a {b: color.hwb(0 30% 40% 0.4)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn one_element() {
        assert_eq!(
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.hwb(0)}\n"
            ),
            "Error: Missing element $whiteness.\
         \n  ,\
         \n2 | a {b: color.hwb(0)}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn two_elements() {
        assert_eq!(
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.hwb(0 30%)}\n"
            ),
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
        runner().err(
            "@use \'sass:color\';\
             \na {b: color.hwb(0 30% \"var(--foo) / 0.4\")}\n"
        ),
        "Error: $blackness: \"var(--foo) / 0.4\" is not a number.\
         \n  ,\
         \n2 | a {b: color.hwb(0 30% \"var(--foo) / 0.4\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
mod whiteness {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_high() {
        assert_eq!(
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.hwb(0 101% 40%)}\n"
            ),
            "Error: $whiteness: Expected 101% to be within 0% and 100%.\
         \n  ,\
         \n2 | a {b: color.hwb(0 101% 40%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_low() {
        assert_eq!(
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.hwb(0 -1% 40%)}\n"
            ),
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
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.hwb(0 \"foo\" 40%)}\n"
            ),
            "Error: $whiteness: \"foo\" is not a number.\
         \n  ,\
         \n2 | a {b: color.hwb(0 \"foo\" 40%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    mod unit {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn none() {
            assert_eq!(
                runner().err(
                    "@use \'sass:color\';\
             \na {b: color.hwb(0 30 40%)}\n"
                ),
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
                runner().err(
                    "@use \'sass:color\';\
             \na {b: color.hwb(0 30px 40%)}\n"
                ),
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
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.hwb(0 var(--c) 40%)}\n"
            ),
            "Error: $whiteness: var(--c) is not a number.\
         \n  ,\
         \n2 | a {b: color.hwb(0 var(--c) 40%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
