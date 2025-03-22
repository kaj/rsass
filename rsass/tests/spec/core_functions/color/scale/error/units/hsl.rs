//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/error/units/hsl.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hsl")
}

mod none {
    use super::runner;

    #[test]
    fn lightness() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(white, $lightness: 1)}\n"
            ),
            "Error: $lightness: Expected 1 to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(white, $lightness: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn saturation() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(white, $saturation: 1)}\n"
            ),
            "Error: $saturation: Expected 1 to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(white, $saturation: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
mod wrong {
    use super::runner;

    #[test]
    fn lightness() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(white, $lightness: 1px)}\n"
            ),
            "Error: $lightness: Expected 1px to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(white, $lightness: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn saturation() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(white, $saturation: 1px)}\n"
            ),
            "Error: $saturation: Expected 1px to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(white, $saturation: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
