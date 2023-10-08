//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/error/value.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("value")
}

mod function {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn boolean() {
        assert_eq!(
            runner().err(
                "@function a() {@return true}\
             \nb {c: calc(a())}\n"
            ),
            "Error: Value true can\'t be used in a calculation.\
         \n  ,\
         \n2 | b {c: calc(a())}\
         \n  |            ^^^\
         \n  \'\
         \n  input.scss 2:12  root stylesheet",
        );
    }
    #[test]
    fn color() {
        assert_eq!(
            runner().err(
                "@function a() {@return blue}\
             \nb {c: calc(a())}\n"
            ),
            "Error: Value blue can\'t be used in a calculation.\
         \n  ,\
         \n2 | b {c: calc(a())}\
         \n  |            ^^^\
         \n  \'\
         \n  input.scss 2:12  root stylesheet",
        );
    }
    #[test]
    fn function() {
        assert_eq!(
        runner().err(
            "@use \'sass:meta\';\
             \n@function a() {@return meta.get-function(\"get-function\", $module: \"meta\")}\
             \nb {c: calc(a())}\n"
        ),
        "Error: Value get-function(\"get-function\") can\'t be used in a calculation.\
         \n  ,\
         \n3 | b {c: calc(a())}\
         \n  |            ^^^\
         \n  \'\
         \n  input.scss 3:12  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn list() {
        assert_eq!(
            runner().err(
                "@function a() {@return 1 2 3}\
             \nb {c: calc(a())}\n"
            ),
            "Error: Value 1 2 3 can\'t be used in a calculation.\
         \n  ,\
         \n2 | b {c: calc(a())}\
         \n  |            ^^^\
         \n  \'\
         \n  input.scss 2:12  root stylesheet",
        );
    }
    #[test]
    fn map() {
        assert_eq!(
            runner().err(
                "@function a() {@return (b: c)}\
             \nd {e: calc(a())}\n"
            ),
            "Error: Value (b: c) can\'t be used in a calculation.\
         \n  ,\
         \n2 | d {e: calc(a())}\
         \n  |            ^^^\
         \n  \'\
         \n  input.scss 2:12  root stylesheet",
        );
    }
    #[test]
    fn null() {
        assert_eq!(
            runner().err(
                "@function a() {@return null}\
             \nb {c: calc(a())}\n"
            ),
            "Error: Value null can\'t be used in a calculation.\
         \n  ,\
         \n2 | b {c: calc(a())}\
         \n  |            ^^^\
         \n  \'\
         \n  input.scss 2:12  root stylesheet",
        );
    }
    #[test]
    fn quoted_string() {
        assert_eq!(
            runner().err(
                "@function a() {@return \"foo\"}\
             \nb {c: calc(a())}\n"
            ),
            "Error: Value \"foo\" can\'t be used in a calculation.\
         \n  ,\
         \n2 | b {c: calc(a())}\
         \n  |            ^^^\
         \n  \'\
         \n  input.scss 2:12  root stylesheet",
        );
    }
}
mod variable {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn boolean() {
        assert_eq!(
            runner().err(
                "$a: true;\
             \nb {c: calc($a)}\n"
            ),
            "Error: Value true can\'t be used in a calculation.\
         \n  ,\
         \n2 | b {c: calc($a)}\
         \n  |            ^^\
         \n  \'\
         \n  input.scss 2:12  root stylesheet",
        );
    }
    #[test]
    fn color() {
        assert_eq!(
            runner().err(
                "$a: blue;\
             \nb {c: calc($a)}\n"
            ),
            "Error: Value blue can\'t be used in a calculation.\
         \n  ,\
         \n2 | b {c: calc($a)}\
         \n  |            ^^\
         \n  \'\
         \n  input.scss 2:12  root stylesheet",
        );
    }
    #[test]
    fn function() {
        assert_eq!(
        runner().err(
            "@use \'sass:meta\';\
             \n$a: meta.get-function(\"get-function\", $module: \"meta\");\
             \nb {c: calc($a)}\n"
        ),
        "Error: Value get-function(\"get-function\") can\'t be used in a calculation.\
         \n  ,\
         \n3 | b {c: calc($a)}\
         \n  |            ^^\
         \n  \'\
         \n  input.scss 3:12  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn list() {
        assert_eq!(
            runner().err(
                "$a: 1 2 3;\
             \nb {c: calc($a)}\n"
            ),
            "Error: Value 1 2 3 can\'t be used in a calculation.\
         \n  ,\
         \n2 | b {c: calc($a)}\
         \n  |            ^^\
         \n  \'\
         \n  input.scss 2:12  root stylesheet",
        );
    }
    #[test]
    fn map() {
        assert_eq!(
            runner().err(
                "$a: (b: c);\
             \nd {e: calc($a)}\n"
            ),
            "Error: Value (b: c) can\'t be used in a calculation.\
         \n  ,\
         \n2 | d {e: calc($a)}\
         \n  |            ^^\
         \n  \'\
         \n  input.scss 2:12  root stylesheet",
        );
    }
    #[test]
    fn null() {
        assert_eq!(
            runner().err(
                "$a: null;\
             \nb {c: calc($a)}\n"
            ),
            "Error: Value null can\'t be used in a calculation.\
         \n  ,\
         \n2 | b {c: calc($a)}\
         \n  |            ^^\
         \n  \'\
         \n  input.scss 2:12  root stylesheet",
        );
    }
    #[test]
    fn quoted_string() {
        assert_eq!(
            runner().err(
                "$a: \"foo\";\
             \nb {c: calc($a)}\n"
            ),
            "Error: Value \"foo\" can\'t be used in a calculation.\
         \n  ,\
         \n2 | b {c: calc($a)}\
         \n  |            ^^\
         \n  \'\
         \n  input.scss 2:12  root stylesheet",
        );
    }
}
