//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust/error/units/hwb.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hwb")
}

mod none {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn blackness() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.adjust(black, $blackness: 1)}\n"
            ),
            "Error: $blackness: Expected 1 to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.adjust(black, $blackness: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn whiteness() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.adjust(white, $whiteness: 1)}\n"
            ),
            "Error: $whiteness: Expected 1 to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.adjust(white, $whiteness: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
mod wrong {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn blackness() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.adjust(black, $blackness: 1px)}\n"
            ),
            "Error: $blackness: Expected 1px to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.adjust(black, $blackness: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn whiteness() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.adjust(white, $whiteness: 1px)}\n"
            ),
            "Error: $whiteness: Expected 1px to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.adjust(white, $whiteness: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
