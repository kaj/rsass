//! Tests auto-converted from "sass-spec/spec/core_functions/color/alpha.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("alpha")
}

mod color {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn max() {
        assert_eq!(
            runner().ok("a {b: alpha(red)}\n"),
            "a {\
         \n  b: 1;\
         \n}\n"
        );
    }
    #[test]
    fn middle() {
        assert_eq!(
            runner().ok("a {b: alpha(rgba(red, 0.42))}\n"),
            "a {\
         \n  b: 0.42;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("a {b: alpha(rgba(red, 0))}\n"),
            "a {\
         \n  b: 0;\
         \n}\n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            runner().ok("a {b: alpha($color: rgba(red, 0.73))}\n"),
            "a {\
         \n  b: 0.73;\
         \n}\n"
        );
    }
}
mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn quoted_string() {
        assert_eq!(
            runner().err("a {b: alpha(\"c=d\")}\n"),
            "Error: $color: \"c=d\" is not a color.\
         \n  ,\
         \n1 | a {b: alpha(\"c=d\")}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: alpha()}\n"),
            "Error: Missing argument $color.\
         \n  ,\
         \n1 | a {b: alpha()}\
         \n  |       ^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: alpha(red, green)}\n"),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,\
         \n1 | a {b: alpha(red, green)}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err("a {b: alpha(1)}\n"),
            "Error: $color: 1 is not a color.\
         \n  ,\
         \n1 | a {b: alpha(1)}\
         \n  |       ^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    mod unquoted_string {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn no_equals() {
            assert_eq!(
                runner().err("a {b: alpha(cd)}\n"),
                "Error: $color: cd is not a color.\
         \n  ,\
         \n1 | a {b: alpha(cd)}\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn non_identifier_before_equals() {
            assert_eq!(
                runner().err("a {b: alpha(unquote(\"1=c\"))}\n"),
                "Error: $color: 1=c is not a color.\
         \n  ,\
         \n1 | a {b: alpha(unquote(\"1=c\"))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
    mod with_module {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // missing error
        fn test_type() {
            assert_eq!(
                runner().err(
                    "@use \'sass:color\';\
             \na {b: color.opacity(var(--c))}\n"
                ),
                "Error: $color: var(--c) is not a color.\
         \n  ,\
         \n2 | a {b: color.opacity(var(--c))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
}
mod filter {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn multi_args() {
        assert_eq!(
            runner().ok("a {b: alpha(c=d, e=f, g=h)}\n"),
            "a {\
         \n  b: alpha(c=d, e=f, g=h);\
         \n}\n"
        );
    }
    #[test]
    fn one_arg() {
        assert_eq!(
            runner().ok("a {b: alpha(c=d)}\n"),
            "a {\
         \n  b: alpha(c=d);\
         \n}\n"
        );
    }
    #[test]
    fn space_before_equals() {
        assert_eq!(
            runner().ok("a {b: alpha(unquote(\"c = d\"))}\n"),
            "a {\
         \n  b: alpha(c = d);\
         \n}\n"
        );
    }
}
mod opacity {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn filter() {
        assert_eq!(
            runner().ok("a {b: opacity(10%)}\n"),
            "a {\
         \n  b: opacity(10%);\
         \n}\n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            runner().ok("a {b: opacity($color: rgba(red, 0.2))}\n"),
            "a {\
         \n  b: 0.2;\
         \n}\n"
        );
    }
    #[test]
    fn positional() {
        assert_eq!(
            runner().ok("a {b: opacity(rgba(red, 0.2))}\n"),
            "a {\
         \n  b: 0.2;\
         \n}\n"
        );
    }
    #[test]
    fn with_calc() {
        assert_eq!(
            runner().ok("a {b: opacity(calc(1 + 2))}\n"),
            "a {\
         \n  b: opacity(3);\
         \n}\n"
        );
    }
    #[test]
    fn with_css_var() {
        assert_eq!(
            runner().ok("a {b: opacity(var(--c))}\n"),
            "a {\
         \n  b: opacity(var(--c));\
         \n}\n"
        );
    }
    #[test]
    fn with_unquoted_calc() {
        assert_eq!(
            runner().ok("a {b: opacity(unquote(\'calc(1)\'))}\n"),
            "a {\
         \n  b: opacity(calc(1));\
         \n}\n"
        );
    }
}
