//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/error/four_args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("four_args")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn unit() {
        assert_eq!(
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.hwb(0, 0%, 0%, 0.5px)}\n"
            ),
            "Error: $alpha: Expected 0.5px to have unit \"%\" or no units.\
         \n  ,\
         \n2 | a {b: color.hwb(0, 0%, 0%, 0.5px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn var() {
        assert_eq!(
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.hwb(0, 0%, 0%, var(--c))}\n"
            ),
            "Error: $alpha: var(--c) is not a number.\
         \n  ,\
         \n2 | a {b: color.hwb(0, 0%, 0%, var(--c))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: color.hwb(0, 30%, 101%, 0.5)}\n"
            ),
            "Error: $blackness: Expected 101% to be within 0% and 100%.\
         \n  ,\
         \n2 | a {b: color.hwb(0, 30%, 101%, 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_low() {
        assert_eq!(
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.hwb(0, 30%, -1%, 0.5)}\n"
            ),
            "Error: $blackness: Expected -1% to be within 0% and 100%.\
         \n  ,\
         \n2 | a {b: color.hwb(0, 30%, -1%, 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.hwb(0, 30%, \"foo\", 0.5)}\n"
            ),
            "Error: $blackness: \"foo\" is not a number.\
         \n  ,\
         \n2 | a {b: color.hwb(0, 30%, \"foo\", 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: color.hwb(0, 30%, 40, 0.5)}\n"
                ),
                "Error: $blackness: Expected 40 to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.hwb(0, 30%, 40, 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn wrong() {
            assert_eq!(
                runner().err(
                    "@use \'sass:color\';\
             \na {b: color.hwb(0, 30%, 40px, 0.5)}\n"
                ),
                "Error: $blackness: Expected 40px to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.hwb(0, 30%, 40px, 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: color.hwb(0, 30%, var(--c), 0.5)}\n"
            ),
            "Error: $blackness: var(--c) is not a number.\
         \n  ,\
         \n2 | a {b: color.hwb(0, 30%, var(--c), 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
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
             \na {b: color.hwb(\"foo\", 30%, 40%, 0.5)}\n"
            ),
            "Error: $hue: \"foo\" is not a number.\
         \n  ,\
         \n2 | a {b: color.hwb(\"foo\", 30%, 40%, 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn var() {
        assert_eq!(
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.hwb(var(--c), 30%, 40%, 0.5)}\n"
            ),
            "Error: $hue: var(--c) is not a number.\
         \n  ,\
         \n2 | a {b: color.hwb(var(--c), 30%, 40%, 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
mod whiteness {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_high() {
        assert_eq!(
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.hwb(0, 101%, 40%, 0.5)}\n"
            ),
            "Error: $whiteness: Expected 101% to be within 0% and 100%.\
         \n  ,\
         \n2 | a {b: color.hwb(0, 101%, 40%, 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_low() {
        assert_eq!(
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.hwb(0, -1%, 40%, 0.5)}\n"
            ),
            "Error: $whiteness: Expected -1% to be within 0% and 100%.\
         \n  ,\
         \n2 | a {b: color.hwb(0, -1%, 40%, 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.hwb(0, \"foo\", 40%, 0.5)}\n"
            ),
            "Error: $whiteness: \"foo\" is not a number.\
         \n  ,\
         \n2 | a {b: color.hwb(0, \"foo\", 40%, 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: color.hwb(0, 30, 40%, 0.5)}\n"
                ),
                "Error: $whiteness: Expected 30 to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.hwb(0, 30, 40%, 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn wrong() {
            assert_eq!(
                runner().err(
                    "@use \'sass:color\';\
             \na {b: color.hwb(0, 30px, 40%, 0.5)}\n"
                ),
                "Error: $whiteness: Expected 30px to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.hwb(0, 30px, 40%, 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: color.hwb(0, var(--c), 40%, 0.5)}\n"
            ),
            "Error: $whiteness: var(--c) is not a number.\
         \n  ,\
         \n2 | a {b: color.hwb(0, var(--c), 40%, 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
