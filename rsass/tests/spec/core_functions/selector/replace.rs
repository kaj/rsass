//! Tests auto-converted from "sass-spec/spec/core_functions/selector/replace.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("replace")
}

#[test]
fn complex() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.replace(\"c d\", \"d\", \"e f\")}\n"),
        "a {\
         \n  b: c e f, e c f;\
         \n}\n"
    );
}
#[test]
fn compound() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.replace(\"c.d\", \"c\", \"e\")}\n"),
        "a {\
         \n  b: e.d;\
         \n}\n"
    );
}
mod error {
    use super::runner;

    mod extendee {
        use super::runner;

        mod complex {
            use super::runner;

            #[test]
            fn list() {
                assert_eq!(
                    runner().err(
                        "@use \"sass:selector\";\
             \na {b: selector.replace(\"c\", d e, \"f\")}\n"
                    ),
                    "Error: Can\'t extend complex selector d e.\
         \n  ,\
         \n2 | a {b: selector.replace(\"c\", d e, \"f\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
                );
            }
            #[test]
            fn string() {
                assert_eq!(
                    runner().err(
                        "@use \"sass:selector\";\
             \na {b: selector.replace(\"c\", \"d e\", \"f\")}\n"
                    ),
                    "Error: Can\'t extend complex selector d e.\
         \n  ,\
         \n2 | a {b: selector.replace(\"c\", \"d e\", \"f\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
                );
            }
        }
        #[test]
        fn invalid() {
            assert_eq!(
                runner().err(
                    "@use \"sass:selector\";\
             \na {b: selector.replace(\"c\", \"[d\", \"e\")}\n"
                ),
                "Error: $original: expected more input.\
         \n  ,\
         \n1 | [d\
         \n  |   ^\
         \n  \'\
         \n  - 1:3  root stylesheet\
         \n  ,\
         \n2 | a {b: selector.replace(\"c\", \"[d\", \"e\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn parent() {
            assert_eq!(
                runner().err(
                    "@use \"sass:selector\";\
             \na {b: selector.replace(\"c\", \"&\", \"d\")}\n"
                ),
                "Error: $original: Parent selectors aren\'t allowed here.\
         \n  ,\
         \n1 | &\
         \n  | ^\
         \n  \'\
         \n  - 1:1  root stylesheet\
         \n  ,\
         \n2 | a {b: selector.replace(\"c\", \"&\", \"d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn test_type() {
            assert_eq!(
        runner().err(
            "@use \"sass:selector\";\
             \na {b: selector.replace(\"c\", 1, \"d\")}\n"
        ),
        "Error: $original: 1 is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n2 | a {b: selector.replace(\"c\", 1, \"d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
    }
    mod extender {
        use super::runner;

        #[test]
        fn invalid() {
            assert_eq!(
                runner().err(
                    "@use \"sass:selector\";\
             \na {b: selector.replace(\"c\", \"d\", \"[e\")}\n"
                ),
                "Error: $replacement: expected more input.\
         \n  ,\
         \n1 | [e\
         \n  |   ^\
         \n  \'\
         \n  - 1:3  root stylesheet\
         \n  ,\
         \n2 | a {b: selector.replace(\"c\", \"d\", \"[e\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn parent() {
            assert_eq!(
                runner().err(
                    "@use \"sass:selector\";\
             \na {b: selector.replace(\"c\", \"d\", \"&\")}\n"
                ),
                "Error: $replacement: Parent selectors aren\'t allowed here.\
         \n  ,\
         \n1 | &\
         \n  | ^\
         \n  \'\
         \n  - 1:1  root stylesheet\
         \n  ,\
         \n2 | a {b: selector.replace(\"c\", \"d\", \"&\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn test_type() {
            assert_eq!(
        runner().err(
            "@use \"sass:selector\";\
             \na {b: selector.replace(\"c\", \"d\", 1)}\n"
        ),
        "Error: $replacement: 1 is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n2 | a {b: selector.replace(\"c\", \"d\", 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
    }
    mod selector {
        use super::runner;

        #[test]
        fn invalid() {
            assert_eq!(
                runner().err(
                    "@use \"sass:selector\";\
             \na {b: selector.replace(\"[c\", \"d\", \"e\")}\n"
                ),
                "Error: $selector: expected more input.\
         \n  ,\
         \n1 | [c\
         \n  |   ^\
         \n  \'\
         \n  - 1:3  root stylesheet\
         \n  ,\
         \n2 | a {b: selector.replace(\"[c\", \"d\", \"e\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn parent() {
            assert_eq!(
                runner().err(
                    "@use \"sass:selector\";\
             \na {b: selector.replace(\"&\", \"c\", \"d\")}\n"
                ),
                "Error: $selector: Parent selectors aren\'t allowed here.\
         \n  ,\
         \n1 | &\
         \n  | ^\
         \n  \'\
         \n  - 1:1  root stylesheet\
         \n  ,\
         \n2 | a {b: selector.replace(\"&\", \"c\", \"d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn test_type() {
            assert_eq!(
        runner().err(
            "@use \"sass:selector\";\
             \na {b: selector.replace(1, \"c\", \"d\")}\n"
        ),
        "Error: $selector: 1 is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n2 | a {b: selector.replace(1, \"c\", \"d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
        runner().err(
            "@use \"sass:selector\";\
             \na {b: selector.replace(\"c\", \"d\")}\n"
        ),
        "Error: Missing argument $replacement.\
         \n  ,--> input.scss\
         \n2 | a {b: selector.replace(\"c\", \"d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:selector\
         \n1 | @function replace($selector, $original, $replacement) {\
         \n  |           =========================================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
        runner().err(
            "@use \"sass:selector\";\
             \na {b: selector.replace(\"c\", \"d\", \"e\", \"f\")}\n"
        ),
        "Error: Only 3 arguments allowed, but 4 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: selector.replace(\"c\", \"d\", \"e\", \"f\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:selector\
         \n1 | @function replace($selector, $original, $replacement) {\
         \n  |           =========================================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    #[test]
    fn wrong_name() {
        assert_eq!(
            runner().err(
                "@use \"sass:selector\";\
             \na {b: selector.selector-replace(c, c, d)}\n"
            ),
            "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: selector.selector-replace(c, c, d)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
mod format {
    use super::runner;

    mod input {
        use super::runner;

        mod multiple_extendees {
            use super::runner;

            #[test]
            fn compound() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.replace(\"c.d\", \"c.d\", \".e\")}\n"),
                    "a {\
         \n  b: .e;\
         \n}\n"
                );
            }
            #[test]
            fn list() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.replace(\"c.d\", \"c, .d\", \".e\")}\n"),
                    "a {\
         \n  b: .e;\
         \n}\n"
                );
            }
            #[test]
            fn list_of_compound() {
                assert_eq!(
                    runner().ok(
                        "@use \"sass:selector\";\
             \na {b: selector.replace(\"c.d.e.f\", \"c.d, .e.f\", \".g\")}\n"
                    ),
                    "a {\
         \n  b: .g;\
         \n}\n"
                );
            }
        }
        mod non_string {
            use super::runner;

            #[test]
            fn extendee() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.replace(\"c.d\", (c, \".d\"), \".e\")}\n"),
                    "a {\
         \n  b: .e;\
         \n}\n"
                );
            }
            #[test]
            fn extender() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.replace(\"c\", \"c\", (d, e f))}\n"),
                    "a {\
         \n  b: d, e f;\
         \n}\n"
                );
            }
            #[test]
            fn selector() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.replace((c, d c), \"c\", \"e\")}\n"),
                    "a {\
         \n  b: e, d e;\
         \n}\n"
                );
            }
        }
    }
    #[test]
    fn output() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \n$result: selector.replace(\"c d, e f\", \"g\", \"g\");\
             \na {\
             \n  result: $result;\
             \n  structure: $result == (\"c\" \"d\", \"e\" \"f\");\
             \n}\n"),
            "a {\
         \n  result: c d, e f;\
         \n  structure: true;\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.replace($selector: \"c.d\", $original: \"c\", $replacement: \"e\")}\n"
        ),
        "a {\
         \n  b: e.d;\
         \n}\n"
    );
}
#[test]
fn no_op() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.replace(\"c\", \"d\", \"e\")}\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
#[test]
fn partial_no_op() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.replace(\"c, d\", \"d\", \"e\")}\n"),
        "a {\
         \n  b: c, e;\
         \n}\n"
    );
}
mod selector_pseudo {
    use super::runner;

    #[test]
    fn is() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.replace(\":is(c)\", \"c\", \"d\")}\n"),
            "a {\
         \n  b: :is(d);\
         \n}\n"
        );
    }
    #[test]
    fn matches() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.replace(\":matches(c)\", \"c\", \"d\")}\n"),
            "a {\
         \n  b: :matches(d);\
         \n}\n"
        );
    }
    #[test]
    fn not() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.replace(\":not(c)\", \"c\", \"d\")}\n"),
            "a {\
         \n  b: :not(d);\
         \n}\n"
        );
    }
    #[test]
    fn test_where() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.replace(\":where(c)\", \"c\", \"d\")}\n"),
            "a {\
         \n  b: :where(d);\
         \n}\n"
        );
    }
}
#[test]
fn simple() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.replace(\"c\", \"c\", \"d\")}\n"),
        "a {\
         \n  b: d;\
         \n}\n"
    );
}
