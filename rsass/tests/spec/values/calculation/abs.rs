//! Tests auto-converted from "sass-spec/spec/values/calculation/abs.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("abs")
}

#[test]
fn case_insensitive() {
    assert_eq!(
        runner().ok("a {b: AbS(-2)}\n"),
        "a {\
         \n  b: 2;\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn sass_script_and_variable() {
        assert_eq!(
            runner().err("a {b: abs($number: var(--c))}\n"),
            "Error: $number: var(--c) is not a number.\
         \n  ,\
         \n1 | a {b: abs($number: var(--c))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    mod syntax {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn invalid_arg() {
            assert_eq!(
                runner().err("a {b: abs($)}\n"),
                "Error: Expected identifier.\
         \n  ,\
         \n1 | a {b: abs($)}\
         \n  |            ^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
            );
        }
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: abs()}\n"),
            "Error: Missing argument.\
         \n  ,\
         \n1 | a {b: abs()}\
         \n  |       ^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: abs(1, 2)}\n"),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,\
         \n1 | a {b: abs(1, 2)}\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err("a {b: abs(\"0\")}\n"),
            "Error: $number: \"0\" is not a number.\
         \n  ,\
         \n1 | a {b: abs(\"0\")}\
         \n  |       ^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
mod math {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn slash_as_division() {
        assert_eq!(
            runner().ok("b {\
             \n  a: 2px / abs(1.5);\
             \n}\n"),
            "b {\
         \n  a: 1.3333333333px;\
         \n}\n"
        );
    }
}
#[test]
fn negative() {
    assert_eq!(
        runner().ok("a {b: abs(-5.6)}\n"),
        "a {\
         \n  b: 5.6;\
         \n}\n"
    );
}
#[test]
fn overridden() {
    assert_eq!(
        runner().ok("@function abs($arg) {@return $arg}\
             \na {b: abs(-2)}\n"),
        "a {\
         \n  b: -2;\
         \n}\n"
    );
}
#[test]
fn percentage_warning() {
    assert_eq!(
        runner().ok("a {b: abs(-7.5%)}\n"),
        "a {\
         \n  b: 7.5%;\
         \n}\n"
    );
}
#[test]
fn positive() {
    assert_eq!(
        runner().ok("a {b: abs(1)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn preserves_single_unit() {
    assert_eq!(
        runner().ok("a {b: abs(1 + 1px)}\n"),
        "a {\
         \n  b: 2px;\
         \n}\n"
    );
}
#[test]
fn preserves_units() {
    assert_eq!(
        runner().ok("a {b: abs(-7px / 4em) * 1em}\n"),
        "a {\
         \n  b: 1.75px;\
         \n}\n"
    );
}
#[test]
fn sass_script() {
    assert_eq!(
        runner().ok("a {b: abs($number: -3)}\n"),
        "a {\
         \n  b: 3;\
         \n}\n"
    );
}
#[test]
fn simplification() {
    assert_eq!(
        runner().ok("a {\
             \n  b: abs(1px + 2px - var(--c))\
             \n}\n"),
        "a {\
         \n  b: abs(3px - var(--c));\
         \n}\n"
    );
}
#[test]
fn zero() {
    assert_eq!(
        runner().ok("a {b: abs(0)}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
