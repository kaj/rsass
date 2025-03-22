//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/error/units/hwb.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hwb")
}

mod none {
    use super::runner;

    #[test]
    fn blackness() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(white, $blackness: 1)}\n"
            ),
            "Error: $blackness: Expected 1 to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(white, $blackness: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn whiteness() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(white, $whiteness: 1)}\n"
            ),
            "Error: $whiteness: Expected 1 to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(white, $whiteness: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
mod wrong {
    use super::runner;

    #[test]
    fn blackness() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(white, $blackness: 1px)}\n"
            ),
            "Error: $blackness: Expected 1px to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(white, $blackness: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn whiteness() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(white, $whiteness: 1px)}\n"
            ),
            "Error: $whiteness: Expected 1px to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(white, $whiteness: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
