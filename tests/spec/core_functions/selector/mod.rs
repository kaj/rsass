//! Tests auto-converted from "sass-spec/spec/core_functions/selector"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/core_functions/selector/append.hrx"
mod append {
    #[allow(unused)]
    use super::rsass;
    mod classes {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn double() {
            assert_eq!(
                rsass(
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
                rsass(
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
        #[allow(unused)]
        use super::rsass;

        // Ignoring "invalid", error tests are not supported yet.

        // Ignoring "leading_combinator", error tests are not supported yet.

        // Ignoring "namespace", error tests are not supported yet.

        // Ignoring "parent", error tests are not supported yet.

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.

        // Ignoring "universal", error tests are not supported yet.
    }
    mod format {
        #[allow(unused)]
        use super::rsass;
        mod input {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn initial() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
        #[ignore] // wrong result
        fn output() {
            assert_eq!(
                rsass(
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
            rsass(
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
            rsass(
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
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn descendant() {
            assert_eq!(
                rsass(
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
                rsass(
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
                rsass(
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
}

// Ignoring "extend", not expected to work yet.

// Ignoring "is_superselector", not expected to work yet.

// From "sass-spec/spec/core_functions/selector/nest.hrx"
mod nest {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;
        mod invalid {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "initial", error tests are not supported yet.

            // Ignoring "later", error tests are not supported yet.
        }
        mod parent {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "first_arg", error tests are not supported yet.

            // Ignoring "non_initial", error tests are not supported yet.

            // Ignoring "prefix", error tests are not supported yet.
        }

        // Ignoring "too_few_args", error tests are not supported yet.
        mod test_type {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "initial", error tests are not supported yet.

            // Ignoring "later", error tests are not supported yet.
        }
    }
    mod format {
        #[allow(unused)]
        use super::rsass;
        mod input {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn initial() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn test_final() {
            assert_eq!(
                rsass(
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
                rsass(
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
                rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn alone() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
                    rsass(
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
                    rsass(
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
                    rsass(
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
                    rsass(
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
                    rsass(
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
            rsass(
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
            rsass(
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
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn alone() {
            assert_eq!(
                rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn complex_parent() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
                rsass(
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
                rsass(
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
        #[ignore] // wrong result
        fn multiple() {
            assert_eq!(
                rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn complex_parent() {
                assert_eq!(
                    rsass(
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
            #[ignore] // wrong result
            fn simple_parent() {
                assert_eq!(
                    rsass(
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
                rsass(
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
}

mod parse;

// From "sass-spec/spec/core_functions/selector/replace.hrx"
mod replace {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn complex() {
        assert_eq!(
            rsass(
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
            rsass(
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
        #[allow(unused)]
        use super::rsass;
        mod extendee {
            #[allow(unused)]
            use super::rsass;
            mod complex {
                #[allow(unused)]
                use super::rsass;

                // Ignoring "list", error tests are not supported yet.

                // Ignoring "string", error tests are not supported yet.
            }

            // Ignoring "invalid", error tests are not supported yet.

            // Ignoring "parent", error tests are not supported yet.

            // Ignoring "test_type", error tests are not supported yet.
        }
        mod extender {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "invalid", error tests are not supported yet.

            // Ignoring "parent", error tests are not supported yet.

            // Ignoring "test_type", error tests are not supported yet.
        }
        mod selector {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "invalid", error tests are not supported yet.

            // Ignoring "parent", error tests are not supported yet.

            // Ignoring "test_type", error tests are not supported yet.
        }

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.
    }
    mod format {
        #[allow(unused)]
        use super::rsass;
        mod input {
            #[allow(unused)]
            use super::rsass;
            mod multiple_extendees {
                #[allow(unused)]
                use super::rsass;
                #[test]
                #[ignore] // wrong result
                fn compound() {
                    assert_eq!(
        rsass(
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
        rsass(
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
        rsass(
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
                #[allow(unused)]
                use super::rsass;
                #[test]
                #[ignore] // wrong result
                fn extendee() {
                    assert_eq!(
        rsass(
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
                        rsass(
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
                        rsass(
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
                rsass(
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
        rsass(
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
            rsass(
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
            rsass(
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
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn matches() {
            assert_eq!(
                rsass(
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
                rsass(
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
            rsass(
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
}

// Ignoring "unify", not expected to work yet.
