//! Tests auto-converted from "sass-spec/spec/core_functions/selector/parse"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/core_functions/selector/parse/error.hrx"
mod error {
    #[allow(unused)]
    use super::rsass;

    // Ignoring "inner_comma", error tests are not supported yet.

    // Ignoring "outer_space", error tests are not supported yet.

    // Ignoring "parent", error tests are not supported yet.
    mod parse {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "extra", error tests are not supported yet.

        // Ignoring "invalid", error tests are not supported yet.
    }

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "too_nested", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.
}

// From "sass-spec/spec/core_functions/selector/parse/named.hrx"
#[test]
fn named() {
    assert_eq!(
        rsass(
            "a {b: selector-parse($selector: \"c\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/core_functions/selector/parse/selector.hrx"
mod selector {
    #[allow(unused)]
    use super::rsass;
    mod complex {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn adjacent_sibling() {
            assert_eq!(
                rsass(
                    "$result: selector-parse(\"b + c + d\");\
            \na {\
            \n  result: $result;\
            \n  structure: $result == (b \"+\" c \"+\" d,);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  result: b + c + d;\
        \n  structure: true;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn child() {
            assert_eq!(
                rsass(
                    "$result: selector-parse(\"b > c > d\");\
            \na {\
            \n  result: $result;\
            \n  structure: $result == (b \">\" c \">\" d,);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  result: b > c > d;\
        \n  structure: true;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn descendant() {
            assert_eq!(
                rsass(
                    "$result: selector-parse(\"b c d\");\
            \na {\
            \n  result: $result;\
            \n  structure: $result == (b c d,);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  result: b c d;\
        \n  structure: true;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn sibling() {
            assert_eq!(
                rsass(
                    "$result: selector-parse(\"b ~ c ~ d\");\
            \na {\
            \n  result: $result;\
            \n  structure: $result == (b \"~\" c \"~\" d,);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  result: b ~ c ~ d;\
        \n  structure: true;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn compound() {
        assert_eq!(
            rsass(
                "$result: selector-parse(\"b.c:d\");\
            \na {\
            \n  result: $result;\
            \n  structure: $result == (append((), \"b.c:d\"),);\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  result: b.c:d;\
        \n  structure: true;\
        \n}\
        \n"
        );
    }
    #[test]
    fn list() {
        assert_eq!(
            rsass(
                "$result: selector-parse(\"b c, d e, f g\");\
            \na {\
            \n  result: $result;\
            \n  structure: $result == (b c, d e, f g);\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  result: b c, d e, f g;\
        \n  structure: true;\
        \n}\
        \n"
        );
    }
    mod simple {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn attribute() {
            assert_eq!(
                rsass(
                    "a {b: selector-parse(\"[c^=d]\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: [c^=d];\
        \n}\
        \n"
            );
        }
        #[test]
        fn class() {
            assert_eq!(
                rsass(
                    "a {b: selector-parse(\".c\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: .c;\
        \n}\
        \n"
            );
        }
        #[test]
        fn id() {
            assert_eq!(
                rsass(
                    "a {b: selector-parse(\"#c\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #c;\
        \n}\
        \n"
            );
        }
        #[test]
        fn placeholder() {
            assert_eq!(
                rsass(
                    "a {b: selector-parse(\"%c\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: %c;\
        \n}\
        \n"
            );
        }
        mod pseudo {
            #[allow(unused)]
            use super::rsass;
            mod class {
                #[allow(unused)]
                use super::rsass;
                #[test]
                #[ignore] // unexepected error
                fn arg() {
                    assert_eq!(
                        rsass(
                            "a {b: selector-parse(\":c(@#$)\")}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  b: :c(@#$);\
        \n}\
        \n"
                    );
                }
                #[test]
                #[ignore] // wrong result
                fn combined_arg() {
                    assert_eq!(
        rsass(
            "$result: selector-parse(\":nth-child(2n+1 of b, c)\");\
            \na {\
            \n  result: $result;\
            \n  structure: $result == (append((), \":nth-child(2n+1 of b, c)\"),);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  result: :nth-child(2n+1 of b, c);\
        \n  structure: true;\
        \n}\
        \n"
    );
                }
                #[test]
                fn no_arg() {
                    assert_eq!(
                        rsass(
                            "a {b: selector-parse(\":c\")}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  b: :c;\
        \n}\
        \n"
                    );
                }
                #[test]
                #[ignore] // wrong result
                fn selector_arg() {
                    assert_eq!(
                        rsass(
                            "$result: selector-parse(\":matches(b, c)\");\
            \na {\
            \n  result: $result;\
            \n  structure: $result == (append((), \":matches(b, c)\"),);\
            \n}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  result: :matches(b, c);\
        \n  structure: true;\
        \n}\
        \n"
                    );
                }
            }
            mod element {
                #[allow(unused)]
                use super::rsass;
                #[test]
                #[ignore] // unexepected error
                fn arg() {
                    assert_eq!(
                        rsass(
                            "a {b: selector-parse(\"::c(@#$)\")}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  b: ::c(@#$);\
        \n}\
        \n"
                    );
                }
                #[test]
                fn no_arg() {
                    assert_eq!(
                        rsass(
                            "a {b: selector-parse(\"::c\")}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  b: ::c;\
        \n}\
        \n"
                    );
                }
                #[test]
                #[ignore] // wrong result
                fn selector_arg() {
                    assert_eq!(
                        rsass(
                            "$result: selector-parse(\"::slotted(b, c)\");\
            \na {\
            \n  result: $result;\
            \n  structure: $result == (append((), \"::slotted(b, c)\"),);\
            \n}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  result: ::slotted(b, c);\
        \n  structure: true;\
        \n}\
        \n"
                    );
                }
            }
        }
        #[test]
        fn test_type() {
            assert_eq!(
                rsass(
                    "a {b: selector-parse(\"c\")}\
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
        fn universal() {
            assert_eq!(
                rsass(
                    "a {b: selector-parse(\"*\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: *;\
        \n}\
        \n"
            );
        }
    }
}

// From "sass-spec/spec/core_functions/selector/parse/structure.hrx"
mod structure {
    #[allow(unused)]
    use super::rsass;
    mod decomposed {
        #[allow(unused)]
        use super::rsass;
        mod complex {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn mixed() {
                assert_eq!(
                    rsass(
                        "a {b: selector-parse(c \"d\" e)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: c d e;\
        \n}\
        \n"
                );
            }
            #[test]
            fn quoted() {
                assert_eq!(
                    rsass(
                        "a {b: selector-parse(\"c\" \"d\" \"e\")}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: c d e;\
        \n}\
        \n"
                );
            }
            #[test]
            fn unquoted() {
                assert_eq!(
                    rsass(
                        "a {b: selector-parse(c d e)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: c d e;\
        \n}\
        \n"
                );
            }
        }
        mod full {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn mixed() {
                assert_eq!(
                    rsass(
                        "a {b: selector-parse((c \"d\", e \"f\"))}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: c d, e f;\
        \n}\
        \n"
                );
            }
            #[test]
            fn quoted() {
                assert_eq!(
                    rsass(
                        "a {b: selector-parse((\"c\" \"d\", \"e\" \"f\"))}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: c d, e f;\
        \n}\
        \n"
                );
            }
            #[test]
            fn unquoted() {
                assert_eq!(
                    rsass(
                        "a {b: selector-parse((c d, e f))}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: c d, e f;\
        \n}\
        \n"
                );
            }
        }
        mod middle {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn mixed() {
                assert_eq!(
                    rsass(
                        "a {b: selector-parse(c \"d, e\" f)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: c d, e f;\
        \n}\
        \n"
                );
            }
            #[test]
            fn quoted() {
                assert_eq!(
                    rsass(
                        "a {b: selector-parse(\"c\" \"d, e\" \"f\")}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: c d, e f;\
        \n}\
        \n"
                );
            }
            #[test]
            fn unquoted() {
                assert_eq!(
                    rsass(
                        "a {b: selector-parse(c unquote(\"d, e\") f)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: c d, e f;\
        \n}\
        \n"
                );
            }
        }
        mod partial {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn mixed() {
                assert_eq!(
                    rsass(
                        "a {b: selector-parse((c d, unquote(\"e f\")))}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: c d, e f;\
        \n}\
        \n"
                );
            }
            #[test]
            fn quoted() {
                assert_eq!(
                    rsass(
                        "a {b: selector-parse((\"c d\", \"e f\"))}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: c d, e f;\
        \n}\
        \n"
                );
            }
            #[test]
            fn unquoted() {
                assert_eq!(
        rsass(
            "a {b: selector-parse((unquote(\"c d\"), unquote(\"e f\")))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c d, e f;\
        \n}\
        \n"
    );
            }
        }
    }
    mod full_string {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn quoted() {
            assert_eq!(
                rsass(
                    "a {b: selector-parse(\"c d, e f\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: c d, e f;\
        \n}\
        \n"
            );
        }
        #[test]
        fn unquoted() {
            assert_eq!(
                rsass(
                    "a {b: selector-parse(unquote(\"c d, e f\"))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: c d, e f;\
        \n}\
        \n"
            );
        }
    }
}
