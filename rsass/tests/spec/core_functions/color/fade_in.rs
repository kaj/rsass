//! Tests auto-converted from "sass-spec/spec/core_functions/color/fade_in.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("fade_in")
}

mod error {
    #[allow(unused)]
    use super::runner;

    mod bounds {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn too_high() {
            assert_eq!(
        runner().err(
            "a {b: fade-in(red, 1.001)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.adjust instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: fade-in(red, 1.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $amount: Expected 1.001 to be within 0 and 1.\
         \n  ,\
         \n1 | a {b: fade-in(red, 1.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
        #[test]
        #[ignore] // wrong error
        fn too_low() {
            assert_eq!(
        runner().err(
            "a {b: fade-in(red, -0.001)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.adjust instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: fade-in(red, -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $amount: Expected -0.001 to be within 0 and 1.\
         \n  ,\
         \n1 | a {b: fade-in(red, -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
        #[test]
        #[ignore] // wrong error
        fn unit() {
            assert_eq!(
        runner().err(
            "// This test covers sass/dart-sass#1745, but should be removed once units are\
             \n// fully forbidden (sass/sass#3374).\
             \na {b: fade-in(red, 50%)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.adjust instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n3 | a {b: fade-in(red, 50%)}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 3:7  root stylesheet\n\
         \nError: $amount: Expected 50% to be within 0 and 1.\
         \n  ,\
         \n3 | a {b: fade-in(red, 50%)}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
    );
        }
    }
    #[test]
    #[ignore] // wrong error
    fn non_legacy() {
        assert_eq!(
        runner().err(
            "a {b: fade-in(color(srgb 1 1 1 / 0.1), 0.1)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.adjust instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: fade-in(color(srgb 1 1 1 / 0.1), 0.1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: fade-in() is only supported for legacy colors. Please use color.adjust() instead with an explicit $space argument.\
         \n  ,\
         \n1 | a {b: fade-in(color(srgb 1 1 1 / 0.1), 0.1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: fade-in(red)}\n"),
            "Error: Missing argument $amount.\
         \n  ,--> input.scss\
         \n1 | a {b: fade-in(red)}\
         \n  |       ^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function fade-in($color, $amount) {\
         \n  |           ======================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: fade-in(red, 0.1, 2)}\n"),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: fade-in(red, 0.1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function fade-in($color, $amount) {\
         \n  |           ======================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    mod test_type {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn alpha() {
            assert_eq!(
        runner().err(
            "a {b: fade-in(red, blue)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.adjust instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: fade-in(red, blue)}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $amount: blue is not a number.\
         \n  ,\
         \n1 | a {b: fade-in(red, blue)}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
        #[test]
        #[ignore] // wrong error
        fn color() {
            assert_eq!(
        runner().err(
            "a {b: fade-in(1, 0.1)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.adjust instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: fade-in(1, 0.1)}\
         \n  |       ^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $color: 1 is not a color.\
         \n  ,\
         \n1 | a {b: fade-in(1, 0.1)}\
         \n  |       ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
    }
}
#[test]
fn max() {
    assert_eq!(
        runner().ok("a {b: fade-in(rgba(red, 0.5), 1)}\n"),
        "a {\
         \n  b: red;\
         \n}\n"
    );
}
#[test]
fn max_remaining() {
    assert_eq!(
        runner().ok("a {b: fade-in(rgba(red, 0.5), 0.5)}\n"),
        "a {\
         \n  b: red;\
         \n}\n"
    );
}
#[test]
fn middle() {
    assert_eq!(
        runner().ok("a {b: fade-in(rgba(red, 0.5), 0.14)}\n"),
        "a {\
         \n  b: rgba(255, 0, 0, 0.64);\
         \n}\n"
    );
}
#[test]
fn min() {
    assert_eq!(
        runner().ok("a {b: fade-in(rgba(red, 0.5), 0)}\n"),
        "a {\
         \n  b: rgba(255, 0, 0, 0.5);\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner()
            .ok("a {b: fade-in($color: rgba(red, 0.5), $amount: 0.14)}\n"),
        "a {\
         \n  b: rgba(255, 0, 0, 0.64);\
         \n}\n"
    );
}
#[test]
fn opacify() {
    assert_eq!(
        runner()
            .ok("a {b: opacify($color: rgba(red, 0.5), $amount: 0.14)}\n"),
        "a {\
         \n  b: rgba(255, 0, 0, 0.64);\
         \n}\n"
    );
}
