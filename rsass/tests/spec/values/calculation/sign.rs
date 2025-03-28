//! Tests auto-converted from "sass-spec/spec/values/calculation/sign.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("sign")
}

#[test]
fn case_insensitive() {
    assert_eq!(
        runner().ok("a {b: sIgN(3)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
mod error {
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn sass_script() {
        assert_eq!(
            runner().err("a {b: sign(7 % 3)}\n"),
            "Error: This operation can\'t be used in a calculation.\
         \n  ,\
         \n1 | a {b: sign(7 % 3)}\
         \n  |              ^\
         \n  \'\
         \n  input.scss 1:14  root stylesheet",
        );
    }
    mod syntax {
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn invalid_arg() {
            assert_eq!(
                runner().err("a {b: sign($)}\n"),
                "Error: Expected identifier.\
         \n  ,\
         \n1 | a {b: sign($)}\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
            );
        }
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: sign()}\n"),
            "Error: Missing argument.\
         \n  ,\
         \n1 | a {b: sign()}\
         \n  |       ^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: sign(0, 0)}\n"),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,\
         \n1 | a {b: sign(0, 0)}\
         \n  |       ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn test_type() {
        assert_eq!(
            runner().err("a {b: sign(\"0\")}\n"),
            "Error: This expression can\'t be used in a calculation.\
         \n  ,\
         \n1 | a {b: sign(\"0\")}\
         \n  |            ^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
}
#[test]
fn nan() {
    assert_eq!(
        runner().ok("a {b: sign(NaN)}\n"),
        "a {\
         \n  b: calc(NaN);\
         \n}\n"
    );
}
#[test]
fn negative() {
    assert_eq!(
        runner().ok("a {b: sign(-5.6)}\n"),
        "a {\
         \n  b: -1;\
         \n}\n"
    );
}
#[test]
fn negative_zero() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.div(1, sign(-0.0))}\n"),
        "a {\
         \n  b: calc(-infinity);\
         \n}\n"
    );
}
#[test]
fn overridden() {
    assert_eq!(
        runner().ok("@function sign($arg) {@return $arg}\
             \na {b: sign(2)}\n"),
        "a {\
         \n  b: 2;\
         \n}\n"
    );
}
#[test]
fn positive() {
    assert_eq!(
        runner().ok("a {b: sign(3)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn preserves_units() {
    assert_eq!(
        runner().ok("a {b: sign(-7px / 4em) * 1em}\n"),
        "a {\
         \n  b: -1px;\
         \n}\n"
    );
}
#[test]
fn simplification() {
    assert_eq!(
        runner().ok("a {\
             \n  b: sign(1px + 2px - var(--c))\
             \n}\n"),
        "a {\
         \n  b: sign(3px - var(--c));\
         \n}\n"
    );
}
#[test]
fn zero() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.div(1, sign(0))}\n"),
        "a {\
         \n  b: calc(infinity);\
         \n}\n"
    );
}
#[test]
fn zero_fuzzy() {
    assert_eq!(
        runner().ok("a {b: sign(0.000000000001)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
