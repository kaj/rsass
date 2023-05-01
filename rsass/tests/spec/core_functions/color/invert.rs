//! Tests auto-converted from "sass-spec/spec/core_functions/color/invert.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("invert")
}

#[test]
fn alpha() {
    assert_eq!(
        runner().ok("a {b: invert(rgba(turquoise, 0.4))}\n"),
        "a {\
         \n  b: rgba(191, 31, 47, 0.4);\
         \n}\n"
    );
}
#[test]
fn black() {
    assert_eq!(
        runner().ok("a {b: invert(black)}\n"),
        "a {\
         \n  b: white;\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;

    mod bounds {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn too_high() {
            assert_eq!(
                runner().err("a {b: invert(red, 100.001%)}\n"),
                "Error: $weight: Expected 100.001% to be within 0% and 100%.\
         \n  ,\
         \n1 | a {b: invert(red, 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn too_low() {
            assert_eq!(
                runner().err("a {b: invert(red, -0.001%)}\n"),
                "Error: $weight: Expected -0.001% to be within 0% and 100%.\
         \n  ,\
         \n1 | a {b: invert(red, -0.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
    #[test]
    fn number_with_weight() {
        assert_eq!(
        runner().err(
            "a {b: invert(1, 50%)}\n"
        ),
        "Error: Only one argument may be passed to the plain-CSS invert() function.\
         \n  ,\
         \n1 | a {b: invert(1, 50%)}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: invert()}\n"),
            "Error: Missing argument $color.\
         \n  ,--> input.scss\
         \n1 | a {b: invert()}\
         \n  |       ^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function invert($color, $weight: 100%) {\
         \n  |           ============================= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: invert(turquoise, 0%, 1)}\n"),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: invert(turquoise, 0%, 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function invert($color, $weight: 100%) {\
         \n  |           ============================= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    mod test_type {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn color() {
            assert_eq!(
                runner().err("a {b: invert(c)}\n"),
                "Error: $color: c is not a color.\
         \n  ,\
         \n1 | a {b: invert(c)}\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn weight() {
            assert_eq!(
                runner().err("a {b: invert(red, c)}\n"),
                "Error: $weight: c is not a number.\
         \n  ,\
         \n1 | a {b: invert(red, c)}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn with_module() {
            assert_eq!(
                runner().err(
                    "@use \'sass:color\';\
             \na {b: color.invert(var(--c))}\n"
                ),
                "Error: $color: var(--c) is not a color.\
         \n  ,\
         \n2 | a {b: color.invert(var(--c))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
}
#[test]
fn gray() {
    assert_eq!(
        runner().ok("a {b: invert(gray)}\n"),
        "a {\
         \n  b: #7f7f7f;\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("a {b: invert($color: turquoise, $weight: 0%)}\n"),
        "a {\
         \n  b: turquoise;\
         \n}\n"
    );
}
#[test]
fn number() {
    assert_eq!(
        runner().ok("a {b: invert(10%)}\n"),
        "a {\
         \n  b: invert(10%);\
         \n}\n"
    );
}
#[test]
fn turquoise() {
    assert_eq!(
        runner().ok("a {b: invert(turquoise)}\n"),
        "a {\
         \n  b: #bf1f2f;\
         \n}\n"
    );
}
mod units {
    #[allow(unused)]
    use super::runner;

    mod weight {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn unitless() {
            assert_eq!(
                runner().ok("a {b: invert(turquoise, 10)}\n"),
                "a {\
         \n  b: #4dcdc0;\
         \n}\n"
            );
        }
        #[test]
        fn unknown() {
            assert_eq!(
                runner().ok("a {b: invert(turquoise, 10px)}\n"),
                "a {\
         \n  b: #4dcdc0;\
         \n}\n"
            );
        }
    }
}
mod weighted {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn high() {
        assert_eq!(
            runner().ok("a {b: invert(turquoise, 92%)}\n"),
            "a {\
         \n  b: #b52e3c;\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("a {b: invert(turquoise, 23%)}\n"),
            "a {\
         \n  b: #5db4ab;\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("a {b: invert(turquoise, 100%)}\n"),
            "a {\
         \n  b: #bf1f2f;\
         \n}\n"
        );
    }
    #[test]
    fn middle() {
        assert_eq!(
            runner().ok("a {b: invert(turquoise, 50%)}\n"),
            "a {\
         \n  b: gray;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("a {b: invert(turquoise, 0%)}\n"),
            "a {\
         \n  b: turquoise;\
         \n}\n"
        );
    }
}
#[test]
fn white() {
    assert_eq!(
        runner().ok("a {b: invert(white)}\n"),
        "a {\
         \n  b: black;\
         \n}\n"
    );
}
#[test]
fn with_calc() {
    assert_eq!(
        runner().ok("a {b: invert(calc(1 + 2))}\n"),
        "a {\
         \n  b: invert(3);\
         \n}\n"
    );
}
#[test]
fn with_css_var() {
    assert_eq!(
        runner().ok("a {b: invert(var(--c))}\n"),
        "a {\
         \n  b: invert(var(--c));\
         \n}\n"
    );
}
#[test]
fn with_unquoted_calc() {
    assert_eq!(
        runner().ok("a {b: invert(unquote(\'calc(1)\'))}\n"),
        "a {\
         \n  b: invert(calc(1));\
         \n}\n"
    );
}
