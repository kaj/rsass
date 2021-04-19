//! Tests auto-converted from "sass-spec/spec/core_functions/selector/append.hrx"

mod classes {
    #[test]
    fn double() {
        assert_eq!(
            crate::rsass(
                "a {b: selector-append(\".c, .d\", \".e, .f\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: .c.e, .c.f, .d.e, .d.f;\
        \n}\
        \n"
        );
    }
    #[test]
    fn single() {
        assert_eq!(
            crate::rsass(
                "a {b: selector-append(\".c\", \".d\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: .c.d;\
        \n}\
        \n"
        );
    }
}
mod error {
    #[test]
    #[ignore] // wrong error
    fn invalid() {
        assert_eq!(
            crate::rsass(
                "a {b: selector-append(\"[c\", \"d\")}\
             \n"
            )
            .unwrap_err(),
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
    #[ignore] // missing error
    fn leading_combinator() {
        assert_eq!(
            crate::rsass(
                "a {b: selector-append(\".c\", \"> .d\")}\
             \n"
            )
            .unwrap_err(),
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
            crate::rsass(
                "a {b: selector-append(\"c\", \"|d\")}\
             \n"
            )
            .unwrap_err(),
            "Error: Can\'t append |d to c.\
         \n  ,\
         \n1 | a {b: selector-append(\"c\", \"|d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn parent() {
        assert_eq!(
            crate::rsass(
                "a {b: selector-append(\".c\", \"&\")}\
             \n"
            )
            .unwrap_err(),
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
    #[ignore] // missing error
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: selector-append()}\
             \n"
            )
            .unwrap_err(),
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
            crate::rsass(
                "a {b: selector-append(\"c\", 1)}\
             \n"
            )
            .unwrap_err(),
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
    #[ignore] // missing error
    fn universal() {
        assert_eq!(
            crate::rsass(
                "a {b: selector-append(\".c\", \"*\")}\
             \n"
            )
            .unwrap_err(),
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
    mod input {
        #[test]
        fn initial() {
            assert_eq!(
                crate::rsass(
                    "a {b: selector-append((c, d e), f)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: cf, d ef;\
        \n}\
        \n"
            );
        }
        #[test]
        fn later() {
            assert_eq!(
                crate::rsass(
                    "a {b: selector-append(c, (d, e f))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: cd, ce f;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn output() {
        assert_eq!(
            crate::rsass(
                "$result: selector-append(\"c d, e f\", \"g\");\
            \na {\
            \n  result: $result;\
            \n  structure: $result == (\"c\" \"dg\", \"e\" \"fg\");\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  result: c dg, e fg;\
        \n  structure: true;\
        \n}\
        \n"
        );
    }
}
#[test]
fn many_args() {
    assert_eq!(
        crate::rsass(
            "a {b: selector-append(\".c\", \".d\", \".e\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: .c.d.e;\
        \n}\
        \n"
    );
}
#[test]
fn one_arg() {
    assert_eq!(
        crate::rsass(
            "a {b: selector-append(\".c.d\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: .c.d;\
        \n}\
        \n"
    );
}
mod suffix {
    #[test]
    fn descendant() {
        assert_eq!(
            crate::rsass(
                "a {b: selector-append(\"c d\", \"e f\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c de f;\
        \n}\
        \n"
        );
    }
    #[test]
    fn multiple() {
        assert_eq!(
            crate::rsass(
                "a {b: selector-append(\".c, .d\", \"e, f\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: .ce, .cf, .de, .df;\
        \n}\
        \n"
        );
    }
    #[test]
    fn single() {
        assert_eq!(
            crate::rsass(
                "a {b: selector-append(\".c\", \"d\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: .cd;\
        \n}\
        \n"
        );
    }
}
