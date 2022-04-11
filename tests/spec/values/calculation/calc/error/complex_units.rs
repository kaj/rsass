//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/error/complex_units.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod denominator {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn from_variable() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \n$a: math.div(1, 2px);\
             \nb {c: calc(1% + $a)}\n"
            ),
            "Error: Number 0.5px^-1 isn\'t compatible with CSS calculations.\
         \n  ,\
         \n3 | b {c: calc(1% + $a)}\
         \n  |            ^^^^^^^\
         \n  \'\
         \n  input.scss 3:12  root stylesheet",
        );
    }
    #[test]
    fn within_calc() {
        assert_eq!(
            runner().err("a {b: calc(1% + 1 / 2px)}\n"),
            "Error: Number 0.5px^-1 isn\'t compatible with CSS calculations.\
         \n  ,\
         \n1 | a {b: calc(1% + 1 / 2px)}\
         \n  |            ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
}
mod multiple_numerator {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn from_variable() {
        assert_eq!(
            runner().err(
                "$a: 1px * 2px;\
             \nb {c: calc(1% + $a)}\n"
            ),
            "Error: Number 2px*px isn\'t compatible with CSS calculations.\
         \n  ,\
         \n2 | b {c: calc(1% + $a)}\
         \n  |            ^^^^^^^\
         \n  \'\
         \n  input.scss 2:12  root stylesheet",
        );
    }
    #[test]
    fn within_calc() {
        assert_eq!(
            runner().err("a {b: calc(1% + 1px * 2px)}\n"),
            "Error: Number 2px*px isn\'t compatible with CSS calculations.\
         \n  ,\
         \n1 | a {b: calc(1% + 1px * 2px)}\
         \n  |            ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
}
mod numerator_and_denominator {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn from_variable() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \n$a: math.div(1s, 2px);\
             \nb {c: calc(1% + $a)}\n"
            ),
            "Error: Number 0.5s/px isn\'t compatible with CSS calculations.\
         \n  ,\
         \n3 | b {c: calc(1% + $a)}\
         \n  |            ^^^^^^^\
         \n  \'\
         \n  input.scss 3:12  root stylesheet",
        );
    }
    #[test]
    fn within_calc() {
        assert_eq!(
            runner().err("a {b: calc(1% + 1s / 2px)}\n"),
            "Error: Number 0.5s/px isn\'t compatible with CSS calculations.\
         \n  ,\
         \n1 | a {b: calc(1% + 1s / 2px)}\
         \n  |            ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
}
