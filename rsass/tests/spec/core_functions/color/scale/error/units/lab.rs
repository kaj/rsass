//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/error/units/lab.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lab")
}

mod none {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn a() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(lab(50% 40% -20%), $a: 1)}\n"
            ),
            "Error: $a: Expected 1 to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(lab(50% 40% -20%), $a: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn b() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(lab(50% 40% -20%), $b: 1)}\n"
            ),
            "Error: $b: Expected 1 to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(lab(50% 40% -20%), $b: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn lightness() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(lab(50% 40% -20%), $lightness: 1)}\n"
            ),
            "Error: $lightness: Expected 1 to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(lab(50% 40% -20%), $lightness: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
    fn a() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(lab(50% 40% -20%), $a: 1px)}\n"
            ),
            "Error: $a: Expected 1px to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(lab(50% 40% -20%), $a: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn b() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(lab(50% 40% -20%), $b: 1px)}\n"
            ),
            "Error: $b: Expected 1px to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(lab(50% 40% -20%), $b: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn lightness() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(lab(50% 40% -20%), $lightness: 1px)}\n"
            ),
            "Error: $lightness: Expected 1px to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(lab(50% 40% -20%), $lightness: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
