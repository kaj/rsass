//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/error/bounds.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("bounds")
}

mod legacy {
    use super::runner;

    #[test]
    fn too_high() {
        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(red, $whiteness: 100.001%)}\n"
        ),
        "Error: $whiteness: Expected 100.001% to be within -100% and 100%.\
         \n  ,\
         \n2 | a {b: color.scale(red, $whiteness: 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    #[test]
    fn too_low() {
        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(red, $saturation: -100.001%)}\n"
        ),
        "Error: $saturation: Expected -100.001% to be within -100% and 100%.\
         \n  ,\
         \n2 | a {b: color.scale(red, $saturation: -100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
}
mod modern {
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn too_high() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(lab(50% -70 60), $b: 100.001%)}\n"
            ),
            "Error: $b: Expected 100.001% to be within -100% and 100%.\
         \n  ,\
         \n2 | a {b: color.scale(lab(50% -70 60), $b: 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn too_low() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(lab(50% -70 60), $a: -100.001%)}\n"
            ),
            "Error: $a: Expected -100.001% to be within -100% and 100%.\
         \n  ,\
         \n2 | a {b: color.scale(lab(50% -70 60), $a: -100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
