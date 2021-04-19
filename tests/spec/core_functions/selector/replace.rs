//! Tests auto-converted from "sass-spec/spec/core_functions/selector/replace.hrx"

#[test]
#[ignore] // wrong result
fn complex() {
    assert_eq!(
        crate::rsass(
            "a {b: selector-replace(\"c d\", \"d\", \"e f\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c e f, e c f;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn compound() {
    assert_eq!(
        crate::rsass(
            "a {b: selector-replace(\"c.d\", \"c\", \"e\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: e.d;\
        \n}\
        \n"
    );
}
mod error {
    mod extendee {
        mod complex {
            #[test]
            #[ignore] // missing error
            fn list() {
                assert_eq!(
                    crate::rsass(
                        "a {b: selector-replace(\"c\", d e, \"f\")}\
             \n"
                    )
                    .unwrap_err(),
                    "Error: Can\'t extend complex selector d e.\
         \n  ,\
         \n1 | a {b: selector-replace(\"c\", d e, \"f\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
                );
            }
            #[test]
            #[ignore] // missing error
            fn string() {
                assert_eq!(
                    crate::rsass(
                        "a {b: selector-replace(\"c\", \"d e\", \"f\")}\
             \n"
                    )
                    .unwrap_err(),
                    "Error: Can\'t extend complex selector d e.\
         \n  ,\
         \n1 | a {b: selector-replace(\"c\", \"d e\", \"f\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
                );
            }
        }
        #[test]
        #[ignore] // missing error
        fn invalid() {
            assert_eq!(
                crate::rsass(
                    "a {b: selector-replace(\"c\", \"[d\", \"e\")}\
             \n"
                )
                .unwrap_err(),
                "Error: $original: expected more input.\
         \n  ,\
         \n1 | [d\
         \n  |   ^\
         \n  \'\
         \n  - 1:3  root stylesheet\
         \n  ,\
         \n1 | a {b: selector-replace(\"c\", \"[d\", \"e\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
            );
        }
        #[test]
        #[ignore] // missing error
        fn parent() {
            assert_eq!(
                crate::rsass(
                    "a {b: selector-replace(\"c\", \"&\", \"d\")}\
             \n"
                )
                .unwrap_err(),
                "Error: $original: Parent selectors aren\'t allowed here.\
         \n  ,\
         \n1 | &\
         \n  | ^\
         \n  \'\
         \n  - 1:1  root stylesheet\
         \n  ,\
         \n1 | a {b: selector-replace(\"c\", \"&\", \"d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
            );
        }
        #[test]
        #[ignore] // missing error
        fn test_type() {
            assert_eq!(
        crate::rsass(
            "a {b: selector-replace(\"c\", 1, \"d\")}\
             \n"
        ).unwrap_err(),
        "Error: $original: 1 is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n1 | a {b: selector-replace(\"c\", 1, \"d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
    );
        }
    }
    mod extender {
        #[test]
        #[ignore] // missing error
        fn invalid() {
            assert_eq!(
                crate::rsass(
                    "a {b: selector-replace(\"c\", \"d\", \"[e\")}\
             \n"
                )
                .unwrap_err(),
                "Error: $replacement: expected more input.\
         \n  ,\
         \n1 | [e\
         \n  |   ^\
         \n  \'\
         \n  - 1:3  root stylesheet\
         \n  ,\
         \n1 | a {b: selector-replace(\"c\", \"d\", \"[e\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
            );
        }
        #[test]
        #[ignore] // missing error
        fn parent() {
            assert_eq!(
                crate::rsass(
                    "a {b: selector-replace(\"c\", \"d\", \"&\")}\
             \n"
                )
                .unwrap_err(),
                "Error: $replacement: Parent selectors aren\'t allowed here.\
         \n  ,\
         \n1 | &\
         \n  | ^\
         \n  \'\
         \n  - 1:1  root stylesheet\
         \n  ,\
         \n1 | a {b: selector-replace(\"c\", \"d\", \"&\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
            );
        }
        #[test]
        #[ignore] // missing error
        fn test_type() {
            assert_eq!(
        crate::rsass(
            "a {b: selector-replace(\"c\", \"d\", 1)}\
             \n"
        ).unwrap_err(),
        "Error: $replacement: 1 is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n1 | a {b: selector-replace(\"c\", \"d\", 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
    );
        }
    }
    mod selector {
        #[test]
        #[ignore] // missing error
        fn invalid() {
            assert_eq!(
                crate::rsass(
                    "a {b: selector-replace(\"[c\", \"d\", \"e\")}\
             \n"
                )
                .unwrap_err(),
                "Error: $selector: expected more input.\
         \n  ,\
         \n1 | [c\
         \n  |   ^\
         \n  \'\
         \n  - 1:3  root stylesheet\
         \n  ,\
         \n1 | a {b: selector-replace(\"[c\", \"d\", \"e\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
            );
        }
        #[test]
        #[ignore] // missing error
        fn parent() {
            assert_eq!(
                crate::rsass(
                    "a {b: selector-replace(\"&\", \"c\", \"d\")}\
             \n"
                )
                .unwrap_err(),
                "Error: $selector: Parent selectors aren\'t allowed here.\
         \n  ,\
         \n1 | &\
         \n  | ^\
         \n  \'\
         \n  - 1:1  root stylesheet\
         \n  ,\
         \n1 | a {b: selector-replace(\"&\", \"c\", \"d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
            );
        }
        #[test]
        #[ignore] // missing error
        fn test_type() {
            assert_eq!(
        crate::rsass(
            "a {b: selector-replace(1, \"c\", \"d\")}\
             \n"
        ).unwrap_err(),
        "Error: $selector: 1 is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n1 | a {b: selector-replace(1, \"c\", \"d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
    );
        }
    }
    #[test]
    #[ignore] // missing error
    fn too_few_args() {
        assert_eq!(
        crate::rsass(
            "a {b: selector-replace(\"c\", \"d\")}\
             \n"
        ).unwrap_err(),
        "Error: Missing argument $replacement.\
         \n  ,--> input.scss\
         \n1 | a {b: selector-replace(\"c\", \"d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:selector\
         \n1 | @function replace($selector, $original, $replacement) {\
         \n  |           =========================================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
    );
    }
    #[test]
    #[ignore] // missing error
    fn too_many_args() {
        assert_eq!(
        crate::rsass(
            "a {b: selector-replace(\"c\", \"d\", \"e\", \"f\")}\
             \n"
        ).unwrap_err(),
        "Error: Only 3 arguments allowed, but 4 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: selector-replace(\"c\", \"d\", \"e\", \"f\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:selector\
         \n1 | @function replace($selector, $original, $replacement) {\
         \n  |           =========================================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
    );
    }
}
mod format {
    mod input {
        mod multiple_extendees {
            #[test]
            #[ignore] // wrong result
            fn compound() {
                assert_eq!(
                    crate::rsass(
                        "a {b: selector-replace(\"c.d\", \"c.d\", \".e\")}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: .e;\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn list() {
                assert_eq!(
                    crate::rsass(
                        "a {b: selector-replace(\"c.d\", \"c, .d\", \".e\")}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: .e;\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn list_of_compound() {
                assert_eq!(
        crate::rsass(
            "a {b: selector-replace(\"c.d.e.f\", \"c.d, .e.f\", \".g\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: .g;\
        \n}\
        \n"
    );
            }
        }
        mod non_string {
            #[test]
            #[ignore] // wrong result
            fn extendee() {
                assert_eq!(
        crate::rsass(
            "a {b: selector-replace(\"c.d\", (c, \".d\"), \".e\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: .e;\
        \n}\
        \n"
    );
            }
            #[test]
            #[ignore] // wrong result
            fn extender() {
                assert_eq!(
                    crate::rsass(
                        "a {b: selector-replace(\"c\", \"c\", (d, e f))}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: d, e f;\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn selector() {
                assert_eq!(
                    crate::rsass(
                        "a {b: selector-replace((c, d c), \"c\", \"e\")}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: e, d e;\
        \n}\
        \n"
                );
            }
        }
    }
    #[test]
    #[ignore] // wrong result
    fn output() {
        assert_eq!(
            crate::rsass(
                "$result: selector-replace(\"c d, e f\", \"g\", \"g\");\
            \na {\
            \n  result: $result;\
            \n  structure: $result == (\"c\" \"d\", \"e\" \"f\");\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  result: c d, e f;\
        \n  structure: true;\
        \n}\
        \n"
        );
    }
}
#[test]
#[ignore] // wrong result
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: selector-replace($selector: \"c.d\", $original: \"c\", $replacement: \"e\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: e.d;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn no_op() {
    assert_eq!(
        crate::rsass(
            "a {b: selector-replace(\"c\", \"d\", \"e\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn partial_no_op() {
    assert_eq!(
        crate::rsass(
            "a {b: selector-replace(\"c, d\", \"d\", \"e\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c, e;\
        \n}\
        \n"
    );
}
mod selector_pseudo {
    #[test]
    #[ignore] // wrong result
    fn matches() {
        assert_eq!(
            crate::rsass(
                "a {b: selector-replace(\":matches(c)\", \"c\", \"d\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: :matches(d);\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn not() {
        assert_eq!(
            crate::rsass(
                "a {b: selector-replace(\":not(c)\", \"c\", \"d\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: :not(d);\
        \n}\
        \n"
        );
    }
}
#[test]
#[ignore] // wrong result
fn simple() {
    assert_eq!(
        crate::rsass(
            "a {b: selector-replace(\"c\", \"c\", \"d\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: d;\
        \n}\
        \n"
    );
}
