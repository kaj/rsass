//! Tests auto-converted from "sass-spec/spec/core_functions/color/change_color/error/units.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod none {
    #[allow(unused)]
    use super::runner;
    #[test]
    #[ignore] // missing error
    fn blackness() {
        assert_eq!(
            runner().err("a {b: change-color(black, $blackness: 1)}\n"),
            "Error: $blackness: Expected 1 to have unit \"%\".\
         \n  ,\
         \n1 | a {b: change-color(black, $blackness: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn whiteness() {
        assert_eq!(
            runner().err("a {b: change-color(white, $whiteness: 1)}\n"),
            "Error: $whiteness: Expected 1 to have unit \"%\".\
         \n  ,\
         \n1 | a {b: change-color(white, $whiteness: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
mod wrong {
    #[allow(unused)]
    use super::runner;
    #[test]
    #[ignore] // missing error
    fn blackness() {
        assert_eq!(
            runner().err("a {b: change-color(black, $blackness: 1px)}\n"),
            "Error: $blackness: Expected 1px to have unit \"%\".\
         \n  ,\
         \n1 | a {b: change-color(black, $blackness: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn whiteness() {
        assert_eq!(
            runner().err("a {b: change-color(white, $whiteness: 1px)}\n"),
            "Error: $whiteness: Expected 1px to have unit \"%\".\
         \n  ,\
         \n1 | a {b: change-color(white, $whiteness: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
