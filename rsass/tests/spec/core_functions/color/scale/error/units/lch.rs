//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/error/units/lch.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lch")
}

mod none {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn chroma() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(lch(30% 70% 200deg), $chroma: 1)}\n"
            ),
            "Error: $chroma: Expected 1 to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(lch(30% 70% 200deg), $chroma: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: color.scale(lch(30% 70% 200deg), $lightness: 1)}\n"
            ),
            "Error: $lightness: Expected 1 to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(lch(30% 70% 200deg), $lightness: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
    fn chroma() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(lch(30% 70% 200deg), $chroma: 1px)}\n"
            ),
            "Error: $chroma: Expected 1px to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(lch(30% 70% 200deg), $chroma: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: color.scale(lch(30% 70% 200deg), $lightness: 1px)}\n"
            ),
            "Error: $lightness: Expected 1px to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(lch(30% 70% 200deg), $lightness: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
