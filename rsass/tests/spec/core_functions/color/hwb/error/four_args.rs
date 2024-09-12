//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/error/four_args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("four_args")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn unit() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
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
}
mod blackness {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.hwb(0, 30%, \"foo\", 0.5)}\n"
            ),
            "Error: Expected blackness channel to be a number, was \"foo\".\
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
                    "@use \"sass:color\";\
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
                    "@use \"sass:color\";\
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
}
mod hue {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.hwb(\"foo\", 30%, 40%, 0.5)}\n"
            ),
            "Error: Expected hue channel to be a number, was \"foo\".\
         \n  ,\
         \n2 | a {b: color.hwb(\"foo\", 30%, 40%, 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
mod whiteness {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.hwb(0, \"foo\", 40%, 0.5)}\n"
            ),
            "Error: Expected whiteness channel to be a number, was \"foo\".\
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
                    "@use \"sass:color\";\
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
                    "@use \"sass:color\";\
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
}
