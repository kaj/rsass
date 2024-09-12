//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_gamut/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

mod color {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.to-gamut(c, $method: local-minde)}\n"
            ),
            "Error: $color: c is not a color.\
         \n  ,\
         \n2 | a {b: color.to-gamut(c, $method: local-minde)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
mod method {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn absent() {
        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(pink)}\n"
        ),
        "Error: $method: color.to-gamut() requires a $method argument for forwards-compatibility with changes in the CSS spec. Suggestion:\n\
         \n$method: local-minde\
         \n  ,\
         \n2 | a {b: color.to-gamut(pink)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn null() {
        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(pink, $method: null)}\n"
        ),
        "Error: $method: color.to-gamut() requires a $method argument for forwards-compatibility with changes in the CSS spec. Suggestion:\n\
         \n$method: local-minde\
         \n  ,\
         \n2 | a {b: color.to-gamut(pink, $method: null)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn quoted() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.to-gamut(pink, $method: \"clip\")}\n"
            ),
            "Error: $method: Expected \"clip\" to be an unquoted string.\
         \n  ,\
         \n2 | a {b: color.to-gamut(pink, $method: \"clip\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.to-gamut(pink, $method: 1)}\n"
            ),
            "Error: $method: 1 is not a string.\
         \n  ,\
         \n2 | a {b: color.to-gamut(pink, $method: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn unknown() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.to-gamut(pink, $method: c)}\n"
            ),
            "Error: Unknown gamut map method \"c\".\
         \n  ,\
         \n2 | a {b: color.to-gamut(pink, $method: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
mod space {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn quoted() {
        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(pink, $space: \"rgb\", $method: local-minde)}\n"
        ),
        "Error: $space: Expected \"rgb\" to be an unquoted string.\
         \n  ,\
         \n2 | a {b: color.to-gamut(pink, $space: \"rgb\", $method: local-minde)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn test_type() {
        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(pink, $space: red, $method: local-minde)}\n"
        ),
        "Error: $space: red is not a string.\
         \n  ,\
         \n2 | a {b: color.to-gamut(pink, $space: red, $method: local-minde)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn unknown() {
        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(pink, $space: c, $method: local-minde)}\n"
        ),
        "Error: $space: Unknown color space \"c\".\
         \n  ,\
         \n2 | a {b: color.to-gamut(pink, $space: c, $method: local-minde)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
}
#[test]
#[ignore] // wrong error
fn too_few_args() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.to-gamut()}\n"
        ),
        "Error: Missing argument $color.\
         \n  ,--> input.scss\
         \n2 | a {b: color.to-gamut()}\
         \n  |       ^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function to-gamut($color, $space: null, $method: null) {\
         \n  |           ============================================= declaration\
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
             \na {b: color.to-gamut(red, rgb, clip, c)}\n"
        ),
        "Error: Only 3 arguments allowed, but 4 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: color.to-gamut(red, rgb, clip, c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function to-gamut($color, $space: null, $method: null) {\
         \n  |           ============================================= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
