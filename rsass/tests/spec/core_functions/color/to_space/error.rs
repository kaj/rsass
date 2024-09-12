//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

#[test]
#[ignore] // wrong error
fn quoted_space() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.to-space(#abc, \"hsl\")}\n"
        ),
        "Error: $space: Expected \"hsl\" to be an unquoted string.\
         \n  ,\
         \n2 | a {b: color.to-space(#abc, \"hsl\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn too_few_args() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.to-space(#abc)}\n"
        ),
        "Error: Missing argument $space.\
         \n  ,--> input.scss\
         \n2 | a {b: color.to-space(#abc)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function to-space($color, $space) {\
         \n  |           ======================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn too_many_args() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.to-space(#abc, rgb, hsl)}\n"
        ),
        "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: color.to-space(#abc, rgb, hsl)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function to-space($color, $space) {\
         \n  |           ======================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
mod test_type {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn color() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.to-space(1, rgb)}\n"
            ),
            "Error: $color: 1 is not a color.\
         \n  ,\
         \n2 | a {b: color.to-space(1, rgb)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn space() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.to-space(#abc, #def)}\n"
            ),
            "Error: $space: #def is not a string.\
         \n  ,\
         \n2 | a {b: color.to-space(#abc, #def)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
#[ignore] // wrong error
fn undefined_space() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.to-space(#abc, c)}\n"
        ),
        "Error: $space: Unknown color space \"c\".\
         \n  ,\
         \n2 | a {b: color.to-space(#abc, c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
