//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust_hue/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

#[test]
#[ignore] // wrong error
fn non_legacy() {
    assert_eq!(
        runner().err(
            "a {b: adjust-hue(lch(0% 0 0deg), 10deg)}\n"
        ),
        "DEPRECATION WARNING: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.adjust instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: adjust-hue(lch(0% 0 0deg), 10deg)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: adjust-hue() is only supported for legacy colors. Please use color.adjust() instead with an explicit $space argument.\
         \n  ,\
         \n1 | a {b: adjust-hue(lch(0% 0 0deg), 10deg)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
fn too_few_args() {
    assert_eq!(
        runner().err("a {b: adjust-hue(red)}\n"),
        "Error: Missing argument $degrees.\
         \n  ,--> input.scss\
         \n1 | a {b: adjust-hue(red)}\
         \n  |       ^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function adjust-hue($color, $degrees) {\
         \n  |           ============================ declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
fn too_many_args() {
    assert_eq!(
        runner().err("a {b: adjust-hue(red, 1, 2)}\n"),
        "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: adjust-hue(red, 1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function adjust-hue($color, $degrees) {\
         \n  |           ============================ declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
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
            "a {b: adjust-hue(1, 2)}\n"
        ),
        "DEPRECATION WARNING: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.adjust instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: adjust-hue(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $color: 1 is not a color.\
         \n  ,\
         \n1 | a {b: adjust-hue(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn hue() {
        assert_eq!(
        runner().err(
            "a {b: adjust-hue(red, blue)}\n"
        ),
        "DEPRECATION WARNING: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.adjust instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: adjust-hue(red, blue)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $degrees: blue is not a number.\
         \n  ,\
         \n1 | a {b: adjust-hue(red, blue)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
}
