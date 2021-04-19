//! Tests auto-converted from "sass-spec/spec/core_functions/selector/nest.hrx"

mod error {
    mod invalid {
        #[test]
        #[ignore] // wrong error
        fn initial() {
            assert_eq!(
                crate::rsass(
                    "a {b: selector-nest(\"[c\")}\
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
         \n1 | a {b: selector-nest(\"[c\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn later() {
            assert_eq!(
                crate::rsass(
                    "a {b: selector-nest(\"c\", \"[d\")}\
             \n"
                )
                .unwrap_err(),
                "Error: expected more input.\
         \n  ,\
         \n1 | [d\
         \n  |   ^\
         \n  \'\
         \n  - 1:3  root stylesheet\
         \n  ,\
         \n1 | a {b: selector-nest(\"c\", \"[d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
    mod parent {
        #[test]
        #[ignore] // missing error
        fn first_arg() {
            assert_eq!(
                crate::rsass(
                    "a {b: selector-nest(\"&\")}\
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
         \n1 | a {b: selector-nest(\"&\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn non_initial() {
            assert_eq!(
        crate::rsass(
            "a {b: selector-nest(\"c\", \"[d]&\")}\
             \n"
        ).unwrap_err(),
        "Error: \"&\" may only used at the beginning of a compound selector.\
         \n  ,\
         \n1 | [d]&\
         \n  |    ^\
         \n  \'\
         \n  - 1:4  root stylesheet\
         \n  ,\
         \n1 | a {b: selector-nest(\"c\", \"[d]&\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
        #[test]
        #[ignore] // missing error
        fn prefix() {
            assert_eq!(
        crate::rsass(
            "a {b: selector-nest(\"c\", \"d&\")}\
             \n"
        ).unwrap_err(),
        "Error: \"&\" may only used at the beginning of a compound selector.\
         \n  ,\
         \n1 | d&\
         \n  |  ^\
         \n  \'\
         \n  - 1:2  root stylesheet\
         \n  ,\
         \n1 | a {b: selector-nest(\"c\", \"d&\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
    }
    #[test]
    #[ignore] // missing error
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: selector-nest()}\
             \n\
             \n"
            )
            .unwrap_err(),
            "Error: $selectors: At least one selector must be passed.\
         \n  ,\
         \n1 | a {b: selector-nest()}\
         \n  |       ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    mod test_type {
        #[test]
        fn initial() {
            assert_eq!(
                crate::rsass(
                    "a {b: selector-nest(1)}\
             \n"
                )
                .unwrap_err(),
                "Error: 1 is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n1 | a {b: selector-nest(1)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn later() {
            assert_eq!(
                crate::rsass(
                    "a {b: selector-nest(\"c\", 1)}\
             \n"
                )
                .unwrap_err(),
                "Error: 1 is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n1 | a {b: selector-nest(\"c\", 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
}
mod format {
    mod input {
        #[test]
        fn initial() {
            assert_eq!(
                crate::rsass(
                    "a {b: selector-nest((c, d e), \"f\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: c f, d e f;\
        \n}\
        \n"
            );
        }
        #[test]
        fn later() {
            assert_eq!(
                crate::rsass(
                    "a {b: selector-nest(\"c\", (d, e f))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: c d, c e f;\
        \n}\
        \n"
            );
        }
    }
}
mod list {
    #[test]
    fn test_final() {
        assert_eq!(
            crate::rsass(
                "a {b: selector-nest(\"c\", \"d, e\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c d, c e;\
        \n}\
        \n"
        );
    }
    #[test]
    fn initial() {
        assert_eq!(
            crate::rsass(
                "a {b: selector-nest(\"c, d\", \"e\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c e, d e;\
        \n}\
        \n"
        );
    }
    #[test]
    fn many() {
        assert_eq!(
            crate::rsass(
                "a {b: selector-nest(\"c, d\", \"e, f\", \"g, h\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c e g, c e h, c f g, c f h, d e g, d e h, d f g, d f h;\
        \n}\
        \n"
        );
    }
    mod parent {
        #[test]
        fn alone() {
            assert_eq!(
                crate::rsass(
                    "a {b: selector-nest(\"c, d\", \"&\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: c, d;\
        \n}\
        \n"
            );
        }
        #[test]
        fn complex() {
            assert_eq!(
                crate::rsass(
                    "a {b: selector-nest(\"c, d\", \"e &.f\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: e c.f, e d.f;\
        \n}\
        \n"
            );
        }
        #[test]
        fn compound() {
            assert_eq!(
                crate::rsass(
                    "a {b: selector-nest(\"c, d\", \"&.e\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: c.e, d.e;\
        \n}\
        \n"
            );
        }
        #[test]
        fn in_one_complex() {
            assert_eq!(
                crate::rsass(
                    "a {b: selector-nest(\"c, d\", \"&.e, f\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: c.e, c f, d.e, d f;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn multiple() {
            assert_eq!(
                crate::rsass(
                    "a {b: selector-nest(\"c, d\", \"&.e &.f\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: c.e c.f, c.e d.f, d.e c.f, d.e d.f;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn selector_pseudo() {
            assert_eq!(
                crate::rsass(
                    "a {b: selector-nest(\"c, d\", \":matches(&)\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: :matches(c, d);\
        \n}\
        \n"
            );
        }
        #[test]
        fn suffix() {
            assert_eq!(
                crate::rsass(
                    "a {b: selector-nest(\"c, d\", \"&e\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: ce, de;\
        \n}\
        \n"
            );
        }
    }
}
#[test]
fn many_args() {
    assert_eq!(
        crate::rsass(
            "a {b: selector-nest(\"c\", \"d\", \"e\", \"f\", \"g\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c d e f g;\
        \n}\
        \n"
    );
}
#[test]
fn one_arg() {
    assert_eq!(
        crate::rsass(
            "a {b: selector-nest(\"c\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \n"
    );
}
mod parent {
    #[test]
    fn alone() {
        assert_eq!(
            crate::rsass(
                "a {b: selector-nest(\"c\", \"&\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c;\
        \n}\
        \n"
        );
    }
    mod complex {
        #[test]
        fn complex_parent() {
            assert_eq!(
                crate::rsass(
                    "a {b: selector-nest(\"c d\", \"e &.f\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: e c d.f;\
        \n}\
        \n"
            );
        }
        #[test]
        fn simple_parent() {
            assert_eq!(
                crate::rsass(
                    "a {b: selector-nest(\"c\", \"d &.e\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: d c.e;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn compound() {
        assert_eq!(
            crate::rsass(
                "a {b: selector-nest(\"c\", \"&.d\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c.d;\
        \n}\
        \n"
        );
    }
    #[test]
    fn in_one_complex() {
        assert_eq!(
            crate::rsass(
                "a {b: selector-nest(\"c\", \"&.d, e\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c.d, c e;\
        \n}\
        \n"
        );
    }
    #[test]
    fn multiple() {
        assert_eq!(
            crate::rsass(
                "a {b: selector-nest(\"c\", \"&.d &.e\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c.d c.e;\
        \n}\
        \n"
        );
    }
    mod selector_pseudo {
        #[test]
        fn complex_parent() {
            assert_eq!(
                crate::rsass(
                    "a {b: selector-nest(\"c d\", \":matches(&)\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: :matches(c d);\
        \n}\
        \n"
            );
        }
        #[test]
        fn simple_parent() {
            assert_eq!(
                crate::rsass(
                    "a {b: selector-nest(\"c\", \":matches(&)\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: :matches(c);\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn suffix() {
        assert_eq!(
            crate::rsass(
                "a {b: selector-nest(\"c\", \"&d\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: cd;\
        \n}\
        \n"
        );
    }
}
