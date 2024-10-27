//! Tests auto-converted from "sass-spec/spec/values/calculation/round/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

mod one_argument {
    #[allow(unused)]
    use super::runner;

    mod sass_script {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn variable_named_argument() {
            assert_eq!(
        runner().err(
            "a {b: round($number: var(--c))}\n"
        ),
        "DEPRECATION WARNING: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse math.round instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: round($number: var(--c))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $number: var(--c) is not a number.\
         \n  ,\
         \n1 | a {b: round($number: var(--c))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
    }
    mod syntax {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn invalid_arg() {
            assert_eq!(
                runner().err("a {b: round($)}\n"),
                "Error: Expected identifier.\
         \n  ,\
         \n1 | a {b: round($)}\
         \n  |              ^\
         \n  \'\
         \n  input.scss 1:14  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // wrong error
    fn test_type() {
        assert_eq!(
        runner().err(
            "a {b: round(\"0\")}\n"
        ),
        "DEPRECATION WARNING: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse math.round instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: round(\"0\")}\
         \n  |       ^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $number: \"0\" is not a number.\
         \n  ,\
         \n1 | a {b: round(\"0\")}\
         \n  |       ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    fn unsimplifiable() {
        assert_eq!(
        runner().err(
            "a {b: round(1px + 2px - var(--c))}\n"
        ),
        "Error: Single argument 3px - var(--c) expected to be simplifiable.\
         \n  ,\
         \n1 | a {b: round(1px + 2px - var(--c))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
}
mod three_argument {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn number_type() {
        assert_eq!(
            runner().err(
                "$wrong_input: \"0\";\
             \na {b: round(nearest, $wrong_input, 0)}\n"
            ),
            "Error: Value \"0\" can\'t be used in a calculation.\
         \n  ,\
         \n2 | a {b: round(nearest, $wrong_input, 0)}\
         \n  |                      ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:22  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn step_type() {
        assert_eq!(
            runner().err(
                "$wrong_input: \"0\";\
             \na {b: round(nearest, 0, $wrong_input)}\n"
            ),
            "Error: Value \"0\" can\'t be used in a calculation.\
         \n  ,\
         \n2 | a {b: round(nearest, 0, $wrong_input)}\
         \n  |                         ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:25  root stylesheet",
        );
    }
    mod strategy {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn operation() {
            assert_eq!(
                runner().err(
                    "a {\
             \n  e: round(10px + 2px, 8px, 9px);\
             \n}"
                ),
                "Error: 12px must be either nearest, up, down or to-zero.\
         \n  ,\
         \n2 |   e: round(10px + 2px, 8px, 9px);\
         \n  |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // wrong error
    fn strategy_type() {
        assert_eq!(
            runner().err(
                "$wrong_input: \"nearest\";\
             \na {b: round($wrong_input, 0, 0)}\n"
            ),
            "Error: Value \"nearest\" can\'t be used in a calculation.\
         \n  ,\
         \n2 | a {b: round($wrong_input, 0, 0)}\
         \n  |             ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:13  root stylesheet",
        );
    }
}
#[test]
fn too_few_args() {
    assert_eq!(
        runner().err("a {b: round()}\n"),
        "Error: Missing argument.\
         \n  ,\
         \n1 | a {b: round()}\
         \n  |       ^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
fn too_many_args() {
    assert_eq!(
        runner().err("a {b: round(1, 2, 3, 4)}\n"),
        "Error: Only 3 arguments allowed, but 4 were passed.\
         \n  ,\
         \n1 | a {b: round(1, 2, 3, 4)}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
mod two_argument {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn missing_step() {
        assert_eq!(
            runner().err("a {b: round(nearest, 5)}\n"),
            "Error: If strategy is not null, step is required.\
         \n  ,\
         \n1 | a {b: round(nearest, 5)}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn sass_script() {
        assert_eq!(
            runner().err("a {b: round(7 % 3, 1)}\n"),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: round(7 % 3, 1)}\
         \n  |       ^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function round($number) {\
         \n  |           ============== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    mod units {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn complex_and_unknown() {
            assert_eq!(
        runner().err(
            "a {b: round(1px*2px, 10%)}\n"
        ),
        "Error: Number calc(2px * 1px) isn\'t compatible with CSS calculations.\
         \n  ,\
         \n1 | a {b: round(1px*2px, 10%)}\
         \n  |             ^^^^^^^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
    );
        }
        #[test]
        #[ignore] // wrong error
        fn known_incompatible() {
            assert_eq!(
                runner().err("a {b: round(10deg, 5px)}\n"),
                "Error: 10deg and 5px are incompatible.\
         \n  ,\
         \n1 | a {b: round(10deg, 5px)}\
         \n  |             ^^^^^ 10deg\
         \n  |                    === 5px\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn real_and_unitless() {
            assert_eq!(
                runner().err("a {b: round(10px, 5)}\n"),
                "Error: 10px and 5 are incompatible.\
         \n  ,\
         \n1 | a {b: round(10px, 5)}\
         \n  |             ^^^^ 10px\
         \n  |                   = 5\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // wrong error
    fn x_type() {
        assert_eq!(
            runner().err("a {b: round(0, \"0\")}\n"),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: round(0, \"0\")}\
         \n  |       ^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function round($number) {\
         \n  |           ============== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn y_type() {
        assert_eq!(
            runner().err("a {b: round(\"0\", 0)}\n"),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: round(\"0\", 0)}\
         \n  |       ^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function round($number) {\
         \n  |           ============== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
