//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/error/three_args.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("three_args")
}

mod blackness {
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.hwb(0, 100%, \"foo\")}\n"
            ),
            "Error: Expected blackness channel to be a number, was \"foo\".\
         \n  ,\
         \n2 | a {b: color.hwb(0, 100%, \"foo\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
mod hue {
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.hwb(\"foo\", 100%, 50%)}\n"
            ),
            "Error: Expected hue channel to be a number, was \"foo\".\
         \n  ,\
         \n2 | a {b: color.hwb(\"foo\", 100%, 50%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
mod whiteness {
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.hwb(0, \"foo\", 50%)}\n"
            ),
            "Error: Expected whiteness channel to be a number, was \"foo\".\
         \n  ,\
         \n2 | a {b: color.hwb(0, \"foo\", 50%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
