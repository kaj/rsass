//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/error/one_arg.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("one_arg")
}

mod alpha {
    use super::runner;

    #[test]
    fn unit() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.hwb(0 0% 0% / 0.5px)}\n"
            ),
            "Error: $alpha: Expected 0.5px to have unit \"%\" or no units.\
         \n  ,\
         \n2 | a {b: color.hwb(0 0% 0% / 0.5px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
mod blackness {
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn test_type() {
        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.hwb(0 30% \"foo\")}\n"
        ),
        "Error: $channels: Expected blackness channel to be a number, was \"foo\".\
         \n  ,\
         \n2 | a {b: color.hwb(0 30% \"foo\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    mod unit {
        use super::runner;

        #[test]
        fn none() {
            assert_eq!(
                runner().err(
                    "@use \"sass:color\";\
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
                    "@use \"sass:color\";\
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
}
mod hue {
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn test_type() {
        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.hwb(\"foo\" 30% 40%)}\n"
        ),
        "Error: $channels: Expected hue channel to be a number, was \"foo\".\
         \n  ,\
         \n2 | a {b: color.hwb(\"foo\" 30% 40%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
}
mod list {
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn bracketed() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.hwb([0 30% 40%])}\n"
            ),
            "Error: $channels: Expected an unbracketed list, was [0 30% 40%]\
         \n  ,\
         \n2 | a {b: color.hwb([0 30% 40%])}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn comma_separated() {
        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.hwb((0, 30%, 40%))}\n"
        ),
        "Error: $channels: Expected a space- or slash-separated list, was (0, 30%, 40%)\
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
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.hwb(())}\n"
            ),
            "Error: $channels: Color component list may not be empty.\
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
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.hwb(0 30% 40% 0.4)}\n"
        ),
        "Error: $channels: The hwb color space has 3 channels but (0 30% 40% 0.4) has 4.\
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
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.hwb(0)}\n"
        ),
        "Error: $channels: The hwb color space has 3 channels but 0 has 1.\
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
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.hwb(0 30%)}\n"
        ),
        "Error: $channels: The hwb color space has 3 channels but (0 30%) has 2.\
         \n  ,\
         \n2 | a {b: color.hwb(0 30%)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
}
#[test]
#[ignore] // wrong error
fn quoted_var_slash() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.hwb(0 30% \"var(--foo) / 0.4\")}\n"
        ),
        "Error: $channels: Expected blackness channel to be a number, was \"var(--foo) / 0.4\".\
         \n  ,\
         \n2 | a {b: color.hwb(0 30% \"var(--foo) / 0.4\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
mod whiteness {
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn test_type() {
        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.hwb(0 \"foo\" 40%)}\n"
        ),
        "Error: $channels: Expected whiteness channel to be a number, was \"foo\".\
         \n  ,\
         \n2 | a {b: color.hwb(0 \"foo\" 40%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    mod unit {
        use super::runner;

        #[test]
        fn none() {
            assert_eq!(
                runner().err(
                    "@use \"sass:color\";\
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
                    "@use \"sass:color\";\
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
}
