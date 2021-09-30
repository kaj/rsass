//! Tests auto-converted from "sass-spec/spec/core_functions/selector/nest.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod error {
    #[allow(unused)]
    use super::runner;

    mod invalid {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn initial() {
            assert_eq!(
                runner().err("a {b: selector-nest(\"[c\")}\n"),
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
                runner().err("a {b: selector-nest(\"c\", \"[d\")}\n"),
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
        #[allow(unused)]
        use super::runner;

        #[test]
        fn first_arg() {
            assert_eq!(
                runner().err("a {b: selector-nest(\"&\")}\n"),
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
        runner().err(
            "a {b: selector-nest(\"c\", \"[d]&\")}\n"
        ),
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
        runner().err(
            "a {b: selector-nest(\"c\", \"d&\")}\n"
        ),
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
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: selector-nest()}\n\n"),
            "Error: $selectors: At least one selector must be passed.\
         \n  ,\
         \n1 | a {b: selector-nest()}\
         \n  |       ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    mod test_type {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn initial() {
            assert_eq!(
                runner().err("a {b: selector-nest(1)}\n"),
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
                runner().err("a {b: selector-nest(\"c\", 1)}\n"),
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
    #[allow(unused)]
    use super::runner;

    mod input {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn initial() {
            assert_eq!(
                runner().ok("a {b: selector-nest((c, d e), \"f\")}\n"),
                "a {\
         \n  b: c f, d e f;\
         \n}\n"
            );
        }
        #[test]
        fn later() {
            assert_eq!(
                runner().ok("a {b: selector-nest(\"c\", (d, e f))}\n"),
                "a {\
         \n  b: c d, c e f;\
         \n}\n"
            );
        }
    }
}
mod list {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn test_final() {
        assert_eq!(
            runner().ok("a {b: selector-nest(\"c\", \"d, e\")}\n"),
            "a {\
         \n  b: c d, c e;\
         \n}\n"
        );
    }
    #[test]
    fn initial() {
        assert_eq!(
            runner().ok("a {b: selector-nest(\"c, d\", \"e\")}\n"),
            "a {\
         \n  b: c e, d e;\
         \n}\n"
        );
    }
    #[test]
    fn many() {
        assert_eq!(
            runner()
                .ok("a {b: selector-nest(\"c, d\", \"e, f\", \"g, h\")}\n"),
            "a {\
         \n  b: c e g, c e h, c f g, c f h, d e g, d e h, d f g, d f h;\
         \n}\n"
        );
    }
    mod parent {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn alone() {
            assert_eq!(
                runner().ok("a {b: selector-nest(\"c, d\", \"&\")}\n"),
                "a {\
         \n  b: c, d;\
         \n}\n"
            );
        }
        #[test]
        fn complex() {
            assert_eq!(
                runner().ok("a {b: selector-nest(\"c, d\", \"e &.f\")}\n"),
                "a {\
         \n  b: e c.f, e d.f;\
         \n}\n"
            );
        }
        #[test]
        fn compound() {
            assert_eq!(
                runner().ok("a {b: selector-nest(\"c, d\", \"&.e\")}\n"),
                "a {\
         \n  b: c.e, d.e;\
         \n}\n"
            );
        }
        #[test]
        fn in_one_complex() {
            assert_eq!(
                runner().ok("a {b: selector-nest(\"c, d\", \"&.e, f\")}\n"),
                "a {\
         \n  b: c.e, c f, d.e, d f;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn multiple() {
            assert_eq!(
                runner().ok("a {b: selector-nest(\"c, d\", \"&.e &.f\")}\n"),
                "a {\
         \n  b: c.e c.f, c.e d.f, d.e c.f, d.e d.f;\
         \n}\n"
            );
        }
        mod selector_pseudo {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn is() {
                assert_eq!(
                    runner()
                        .ok("a {b: selector-nest(\"c, d\", \":is(&)\")}\n"),
                    "a {\
         \n  b: :is(c, d);\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn matches() {
                assert_eq!(
                    runner().ok(
                        "a {b: selector-nest(\"c, d\", \":matches(&)\")}\n"
                    ),
                    "a {\
         \n  b: :matches(c, d);\
         \n}\n"
                );
            }
        }
        #[test]
        fn suffix() {
            assert_eq!(
                runner().ok("a {b: selector-nest(\"c, d\", \"&e\")}\n"),
                "a {\
         \n  b: ce, de;\
         \n}\n"
            );
        }
    }
}
#[test]
fn many_args() {
    assert_eq!(
        runner()
            .ok("a {b: selector-nest(\"c\", \"d\", \"e\", \"f\", \"g\")}\n"),
        "a {\
         \n  b: c d e f g;\
         \n}\n"
    );
}
#[test]
fn one_arg() {
    assert_eq!(
        runner().ok("a {b: selector-nest(\"c\")}\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
mod parent {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn alone() {
        assert_eq!(
            runner().ok("a {b: selector-nest(\"c\", \"&\")}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
    mod complex {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn complex_parent() {
            assert_eq!(
                runner().ok("a {b: selector-nest(\"c d\", \"e &.f\")}\n"),
                "a {\
         \n  b: e c d.f;\
         \n}\n"
            );
        }
        #[test]
        fn simple_parent() {
            assert_eq!(
                runner().ok("a {b: selector-nest(\"c\", \"d &.e\")}\n"),
                "a {\
         \n  b: d c.e;\
         \n}\n"
            );
        }
    }
    #[test]
    fn compound() {
        assert_eq!(
            runner().ok("a {b: selector-nest(\"c\", \"&.d\")}\n"),
            "a {\
         \n  b: c.d;\
         \n}\n"
        );
    }
    #[test]
    fn in_one_complex() {
        assert_eq!(
            runner().ok("a {b: selector-nest(\"c\", \"&.d, e\")}\n"),
            "a {\
         \n  b: c.d, c e;\
         \n}\n"
        );
    }
    #[test]
    fn multiple() {
        assert_eq!(
            runner().ok("a {b: selector-nest(\"c\", \"&.d &.e\")}\n"),
            "a {\
         \n  b: c.d c.e;\
         \n}\n"
        );
    }
    mod selector_pseudo {
        #[allow(unused)]
        use super::runner;

        mod complex_parent {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn is() {
                assert_eq!(
                    runner()
                        .ok("a {b: selector-nest(\"c d\", \":is(&)\")}\n"),
                    "a {\
         \n  b: :is(c d);\
         \n}\n"
                );
            }
            #[test]
            fn matches() {
                assert_eq!(
                    runner().ok(
                        "a {b: selector-nest(\"c d\", \":matches(&)\")}\n"
                    ),
                    "a {\
         \n  b: :matches(c d);\
         \n}\n"
                );
            }
        }
        mod simple_parent {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn is() {
                assert_eq!(
                    runner().ok("a {b: selector-nest(\"c\", \":is(&)\")}\n"),
                    "a {\
         \n  b: :is(c);\
         \n}\n"
                );
            }
            #[test]
            fn matches() {
                assert_eq!(
                    runner()
                        .ok("a {b: selector-nest(\"c\", \":matches(&)\")}\n"),
                    "a {\
         \n  b: :matches(c);\
         \n}\n"
                );
            }
        }
    }
    #[test]
    fn suffix() {
        assert_eq!(
            runner().ok("a {b: selector-nest(\"c\", \"&d\")}\n"),
            "a {\
         \n  b: cd;\
         \n}\n"
        );
    }
}
