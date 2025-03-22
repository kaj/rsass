//! Tests auto-converted from "sass-spec/spec/core_functions/color/change/error/bounds.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("bounds")
}

mod alpha {
    use super::runner;

    #[test]
    fn too_high() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.change(red, $alpha: 1.001)}\n"
            ),
            "Error: $alpha: Expected 1.001 to be within 0 and 1.\
         \n  ,\
         \n2 | a {b: color.change(red, $alpha: 1.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_low() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.change(red, $alpha: -0.001)}\n"
            ),
            "Error: $alpha: Expected -0.001 to be within 0 and 1.\
         \n  ,\
         \n2 | a {b: color.change(red, $alpha: -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn unit() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.change(red, $alpha: 150%)}\n"
            ),
            "Error: $alpha: Expected 150% to be within 0% and 100%.\
         \n  ,\
         \n2 | a {b: color.change(red, $alpha: 150%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
