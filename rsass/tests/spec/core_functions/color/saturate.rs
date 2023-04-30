//! Tests auto-converted from "sass-spec/spec/core_functions/color/saturate.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("saturate")
}

mod error {
    #[allow(unused)]
    use super::runner;

    mod one_arg {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn test_type() {
            assert_eq!(
                runner().err("a {b: saturate(red)}\n"),
                "Error: $amount: red is not a number.\
         \n  ,\
         \n1 | a {b: saturate(red)}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: saturate()}\n"),
            "Error: Missing argument $amount.\
         \n  ,--> input.scss\
         \n1 | a {b: saturate()}\
         \n  |       ^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,\
         \n1 | @function saturate($amount) {\
         \n  |           ================= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: saturate(plum, 1%, 2)}\n"),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: saturate(plum, 1%, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,\
         \n1 | @function saturate($color, $amount) {\
         \n  |           ========================= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    mod two_args {
        #[allow(unused)]
        use super::runner;

        mod bounds {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn too_high() {
                assert_eq!(
        runner().err(
            "a {b: saturate(plum, 100.001)}\n"
        ),
        "Error: $amount: Expected 100.001 to be within 0 and 100.\
         \n  ,\
         \n1 | a {b: saturate(plum, 100.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
            }
            #[test]
            fn too_low() {
                assert_eq!(
                    runner().err("a {b: saturate(plum, -0.001)}\n"),
                    "Error: $amount: Expected -0.001 to be within 0 and 100.\
         \n  ,\
         \n1 | a {b: saturate(plum, -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
                );
            }
        }
        mod test_type {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn color() {
                assert_eq!(
                    runner().err("a {b: saturate(1, 2)}\n"),
                    "Error: $color: 1 is not a color.\
         \n  ,\
         \n1 | a {b: saturate(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
                );
            }
            #[test]
            fn lightness() {
                assert_eq!(
                    runner().err("a {b: saturate(plum, blue)}\n"),
                    "Error: $amount: blue is not a number.\
         \n  ,\
         \n1 | a {b: saturate(plum, blue)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
                );
            }
        }
    }
    mod with_module {
        #[allow(unused)]
        use super::runner;

        mod one_arg {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn test_type() {
                assert_eq!(
                    runner().err(
                        "@use \'sass:color\';\
             \na {b: color.saturate(var(--c))}\n"
                    ),
                    "Error: Missing argument $amount.\
         \n  ,--> input.scss\
         \n2 | a {b: color.saturate(var(--c))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function saturate($color, $amount) {\
         \n  |           ========================= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
                );
            }
        }
    }
}
mod one_arg {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn named() {
        assert_eq!(
            runner().ok("a {b: saturate($amount: 50%)}\n"),
            "a {\
         \n  b: saturate(50%);\
         \n}\n"
        );
    }
    #[test]
    fn unit() {
        assert_eq!(
            runner().ok("a {b: saturate(50%)}\n"),
            "a {\
         \n  b: saturate(50%);\
         \n}\n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            runner().ok("a {b: saturate(1)}\n"),
            "a {\
         \n  b: saturate(1);\
         \n}\n"
        );
    }
    #[test]
    fn with_calc() {
        assert_eq!(
            runner().ok("a {b: saturate(calc(1 + 2))}\n"),
            "a {\
         \n  b: saturate(3);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn with_css_var() {
        assert_eq!(
            runner().ok("a {b: saturate(var(--c))}\n"),
            "a {\
         \n  b: saturate(var(--c));\
         \n}\n"
        );
    }
    #[test]
    fn with_sass_var() {
        assert_eq!(
            runner().ok("$c: 1;\
             \na {b: saturate($c)}\n"),
            "a {\
         \n  b: saturate(1);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn with_unquoted_calc() {
        assert_eq!(
            runner().ok("a {b: saturate(unquote(\'calc(1)\'))}\n"),
            "a {\
         \n  b: saturate(calc(1));\
         \n}\n"
        );
    }
}
mod two_args {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn alpha() {
        assert_eq!(
            runner().ok("a {b: saturate(rgba(plum, 0.5), 100%)}\n"),
            "a {\
         \n  b: rgba(255, 126, 255, 0.5);\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("a {b: saturate(plum, 100%)}\n"),
            "a {\
         \n  b: #ff7eff;\
         \n}\n"
        );
    }
    #[test]
    fn max_remaining() {
        assert_eq!(
            runner().ok("a {b: saturate(plum, 53%)}\n"),
            "a {\
         \n  b: #ff7eff;\
         \n}\n"
        );
    }
    #[test]
    fn middle() {
        assert_eq!(
            runner().ok("a {b: saturate(plum, 14%)}\n"),
            "a {\
         \n  b: #e697e6;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("a {b: saturate(plum, 0%)}\n"),
            "a {\
         \n  b: plum;\
         \n}\n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            runner().ok("a {b: saturate($color: plum, $amount: 14%)}\n"),
            "a {\
         \n  b: #e697e6;\
         \n}\n"
        );
    }
}
