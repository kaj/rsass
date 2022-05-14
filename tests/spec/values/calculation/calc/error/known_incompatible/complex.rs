//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/error/known_incompatible/complex.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("complex")
}

#[test]
fn denominator_and_denominators() {
    assert_eq!(
        runner().err("a {b: calc(1/1px + 1/1px/1px)}\n"),
        "Error: Number 1px^-1 isn\'t compatible with CSS calculations.\
         \n  ,\
         \n1 | a {b: calc(1/1px + 1/1px/1px)}\
         \n  |            ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
fn mismatched_denominators() {
    assert_eq!(
        runner().err("a {b: calc(1/1px/1s + 1/1px/1px)}\n"),
        "Error: Number 1(px*s)^-1 isn\'t compatible with CSS calculations.\
         \n  ,\
         \n1 | a {b: calc(1/1px/1s + 1/1px/1px)}\
         \n  |            ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
fn mismatched_numerators() {
    assert_eq!(
        runner().err("a {b: calc(1px*1s + 1px*1px)}\n"),
        "Error: Number 1px*s isn\'t compatible with CSS calculations.\
         \n  ,\
         \n1 | a {b: calc(1px*1s + 1px*1px)}\
         \n  |            ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
fn numerator_and_denominator() {
    assert_eq!(
        runner().err("a {b: calc(1px + 1/1px)}\n"),
        "Error: Number 1px^-1 isn\'t compatible with CSS calculations.\
         \n  ,\
         \n1 | a {b: calc(1px + 1/1px)}\
         \n  |            ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
fn numerator_and_numerators() {
    assert_eq!(
        runner().err("a {b: calc(1px + 1px*1px)}\n"),
        "Error: Number 1px*px isn\'t compatible with CSS calculations.\
         \n  ,\
         \n1 | a {b: calc(1px + 1px*1px)}\
         \n  |            ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
mod unitless {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn and_denominator() {
        assert_eq!(
            runner().err("a {b: calc(1 + 1/1px)}\n"),
            "Error: Number 1px^-1 isn\'t compatible with CSS calculations.\
         \n  ,\
         \n1 | a {b: calc(1 + 1/1px)}\
         \n  |            ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn and_numerator() {
        assert_eq!(
            runner().err("a {b: calc(1 + 1px)}\n"),
            "Error: 1 and 1px are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1 + 1px)}\
         \n  |            ^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
}
