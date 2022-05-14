//! Tests auto-converted from "sass-spec/spec/core_functions/selector/append.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("append")
}

mod classes {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn double() {
        assert_eq!(
            runner().ok("a {b: selector-append(\".c, .d\", \".e, .f\")}\n"),
            "a {\
         \n  b: .c.e, .c.f, .d.e, .d.f;\
         \n}\n"
        );
    }
    #[test]
    fn single() {
        assert_eq!(
            runner().ok("a {b: selector-append(\".c\", \".d\")}\n"),
            "a {\
         \n  b: .c.d;\
         \n}\n"
        );
    }
}
mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn invalid() {
        assert_eq!(
            runner().err("a {b: selector-append(\"[c\", \"d\")}\n"),
            "Error: expected more input.\
         \n  ,\
         \n1 | [c\
         \n  |   ^\
         \n  \'\
         \n  - 1:3  root stylesheet\
         \n  ,\
         \n1 | a {b: selector-append(\"[c\", \"d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn leading_combinator() {
        assert_eq!(
            runner().err("a {b: selector-append(\".c\", \"> .d\")}\n"),
            "Error: Can\'t append > .d to .c.\
         \n  ,\
         \n1 | a {b: selector-append(\".c\", \"> .d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn namespace() {
        assert_eq!(
            runner().err("a {b: selector-append(\"c\", \"|d\")}\n"),
            "Error: Can\'t append |d to c.\
         \n  ,\
         \n1 | a {b: selector-append(\"c\", \"|d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn parent() {
        assert_eq!(
            runner().err("a {b: selector-append(\".c\", \"&\")}\n"),
            "Error: Parent selectors aren\'t allowed here.\
         \n  ,\
         \n1 | &\
         \n  | ^\
         \n  \'\
         \n  - 1:1  root stylesheet\
         \n  ,\
         \n1 | a {b: selector-append(\".c\", \"&\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: selector-append()}\n"),
            "Error: $selectors: At least one selector must be passed.\
         \n  ,\
         \n1 | a {b: selector-append()}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err("a {b: selector-append(\"c\", 1)}\n"),
            "Error: 1 is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n1 | a {b: selector-append(\"c\", 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn universal() {
        assert_eq!(
            runner().err("a {b: selector-append(\".c\", \"*\")}\n"),
            "Error: Can\'t append * to .c.\
         \n  ,\
         \n1 | a {b: selector-append(\".c\", \"*\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
mod format {
    #[allow(unused)]
    use super::runner;

    mod input {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn initial() {
            assert_eq!(
                runner().ok("a {b: selector-append((c, d e), f)}\n"),
                "a {\
         \n  b: cf, d ef;\
         \n}\n"
            );
        }
        #[test]
        fn later() {
            assert_eq!(
                runner().ok("a {b: selector-append(c, (d, e f))}\n"),
                "a {\
         \n  b: cd, ce f;\
         \n}\n"
            );
        }
    }
    #[test]
    fn output() {
        assert_eq!(
            runner().ok("$result: selector-append(\"c d, e f\", \"g\");\
             \na {\
             \n  result: $result;\
             \n  structure: $result == (\"c\" \"dg\", \"e\" \"fg\");\
             \n}\n"),
            "a {\
         \n  result: c dg, e fg;\
         \n  structure: true;\
         \n}\n"
        );
    }
}
#[test]
fn many_args() {
    assert_eq!(
        runner().ok("a {b: selector-append(\".c\", \".d\", \".e\")}\n"),
        "a {\
         \n  b: .c.d.e;\
         \n}\n"
    );
}
#[test]
fn one_arg() {
    assert_eq!(
        runner().ok("a {b: selector-append(\".c.d\")}\n"),
        "a {\
         \n  b: .c.d;\
         \n}\n"
    );
}
mod suffix {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn descendant() {
        assert_eq!(
            runner().ok("a {b: selector-append(\"c d\", \"e f\")}\n"),
            "a {\
         \n  b: c de f;\
         \n}\n"
        );
    }
    #[test]
    fn multiple() {
        assert_eq!(
            runner().ok("a {b: selector-append(\".c, .d\", \"e, f\")}\n"),
            "a {\
         \n  b: .ce, .cf, .de, .df;\
         \n}\n"
        );
    }
    #[test]
    fn single() {
        assert_eq!(
            runner().ok("a {b: selector-append(\".c\", \"d\")}\n"),
            "a {\
         \n  b: .cd;\
         \n}\n"
        );
    }
}
