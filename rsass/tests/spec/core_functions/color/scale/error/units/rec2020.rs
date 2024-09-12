//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/error/units/rec2020.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rec2020")
}

mod none {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn blue() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(color(rec2020 0.2 0.5 0.7), $blue: 1)}\n"
            ),
            "Error: $blue: Expected 1 to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(color(rec2020 0.2 0.5 0.7), $blue: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn green() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(color(rec2020 0.2 0.5 0.7), $green: 1)}\n"
            ),
            "Error: $green: Expected 1 to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(color(rec2020 0.2 0.5 0.7), $green: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn red() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(color(rec2020 0.2 0.5 0.7), $red: 1)}\n"
            ),
            "Error: $red: Expected 1 to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(color(rec2020 0.2 0.5 0.7), $red: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
mod wrong {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn blue() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(color(rec2020 0.2 0.5 0.7), $blue: 1px)}\n"
            ),
            "Error: $blue: Expected 1px to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(color(rec2020 0.2 0.5 0.7), $blue: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn green() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(color(rec2020 0.2 0.5 0.7), $green: 1px)}\n"
            ),
            "Error: $green: Expected 1px to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(color(rec2020 0.2 0.5 0.7), $green: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn red() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(color(rec2020 0.2 0.5 0.7), $red: 1px)}\n"
            ),
            "Error: $red: Expected 1px to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(color(rec2020 0.2 0.5 0.7), $red: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
