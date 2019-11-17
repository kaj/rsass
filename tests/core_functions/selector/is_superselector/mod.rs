//! Tests auto-converted from "sass-spec/spec/core_functions/selector/is_superselector"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/core_functions/selector/is_superselector/complex.hrx"
mod complex {
    #[allow(unused)]
    use super::rsass;
    mod adjacent_sibling {
        #[allow(unused)]
        use super::rsass;
        mod multiple {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn first() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"d + c\", \"d + e + c\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: false;\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn in_sub() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"c\", \"d + e + c\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: true;\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn neither() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"f + c\", \"d + e + c\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: false;\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn second() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"e + c\", \"d + e + c\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: true;\
                     \n}\
                     \n"
                );
            }
        }
        mod single {
            #[allow(unused)]
            use super::rsass;
            mod in_both {
                #[allow(unused)]
                use super::rsass;
                #[test]
                #[ignore] // wrong result
                fn equal() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"c + d\", \"c + d\")}\
                             \n"
                        )
                        .unwrap(),
                        "a {\
                         \n  b: true;\
                         \n}\
                         \n"
                    );
                }
                #[test]
                #[ignore] // wrong result
                fn subset() {
                    assert_eq!(
        rsass(
            "a {b: is-superselector(\"c + d\", \"c.e + d.f\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
                }
                #[test]
                #[ignore] // wrong result
                fn superset() {
                    assert_eq!(
        rsass(
            "a {b: is-superselector(\"c.e + d.f\", \"c + d\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
                }
            }
            #[test]
            #[ignore] // wrong result
            fn in_sub() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"c\", \"d + c\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: true;\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn in_super() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"c + d\", \"d\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: false;\
                     \n}\
                     \n"
                );
            }
        }
    }
    mod child {
        #[allow(unused)]
        use super::rsass;
        mod multiple {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn first() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"d > c\", \"d > e > c\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: false;\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn in_sub() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"c\", \"d > e > c\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: true;\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn neither() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"f > c\", \"d > e > c\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: false;\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn second() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"e > c\", \"d > e > c\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: true;\
                     \n}\
                     \n"
                );
            }
        }
        mod single {
            #[allow(unused)]
            use super::rsass;
            mod in_both {
                #[allow(unused)]
                use super::rsass;
                #[test]
                #[ignore] // wrong result
                fn equal() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"c > d\", \"c > d\")}\
                             \n"
                        )
                        .unwrap(),
                        "a {\
                         \n  b: true;\
                         \n}\
                         \n"
                    );
                }
                #[test]
                #[ignore] // wrong result
                fn subset() {
                    assert_eq!(
        rsass(
            "a {b: is-superselector(\"c > d\", \"c.e > d.f\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
                }
                #[test]
                #[ignore] // wrong result
                fn superset() {
                    assert_eq!(
        rsass(
            "a {b: is-superselector(\"c.e > d.f\", \"c > d\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
                }
            }
            #[test]
            #[ignore] // wrong result
            fn in_sub() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"c\", \"d > c\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: true;\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn in_super() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"c > d\", \"d\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: false;\
                     \n}\
                     \n"
                );
            }
        }
    }
    mod descendant {
        #[allow(unused)]
        use super::rsass;
        mod and_child {
            #[allow(unused)]
            use super::rsass;
            mod multiple {
                #[allow(unused)]
                use super::rsass;
                #[test]
                #[ignore] // wrong result
                fn first() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"d c\", \"d > e > c\")}\
                             \n"
                        )
                        .unwrap(),
                        "a {\
                         \n  b: true;\
                         \n}\
                         \n"
                    );
                }
                #[test]
                #[ignore] // wrong result
                fn neither() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"f c\", \"d > e > c\")}\
                             \n"
                        )
                        .unwrap(),
                        "a {\
                         \n  b: false;\
                         \n}\
                         \n"
                    );
                }
                #[test]
                #[ignore] // wrong result
                fn second() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"e c\", \"d > e > c\")}\
                             \n"
                        )
                        .unwrap(),
                        "a {\
                         \n  b: true;\
                         \n}\
                         \n"
                    );
                }
            }
            #[test]
            #[ignore] // wrong result
            fn sub() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"d > c\", \"d c\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: false;\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn test_super() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"d c\", \"d > c\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: true;\
                     \n}\
                     \n"
                );
            }
        }
        mod multiple {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn in_sub() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"c\", \"d e c\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: true;\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn match_first() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"d c\", \"d e c\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: true;\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn match_neither() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"f c\", \"d e c\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: false;\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn match_second() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"e c\", \"d e c\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: true;\
                     \n}\
                     \n"
                );
            }
        }
        mod single {
            #[allow(unused)]
            use super::rsass;
            mod in_both {
                #[allow(unused)]
                use super::rsass;
                #[test]
                #[ignore] // wrong result
                fn equal() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"c d\", \"c d\")}\
                             \n"
                        )
                        .unwrap(),
                        "a {\
                         \n  b: true;\
                         \n}\
                         \n"
                    );
                }
                #[test]
                #[ignore] // wrong result
                fn subset() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"c d\", \"c.e d.f\")}\
                             \n"
                        )
                        .unwrap(),
                        "a {\
                         \n  b: true;\
                         \n}\
                         \n"
                    );
                }
                #[test]
                #[ignore] // wrong result
                fn superset() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"c.e d.f\", \"c d\")}\
                             \n"
                        )
                        .unwrap(),
                        "a {\
                         \n  b: false;\
                         \n}\
                         \n"
                    );
                }
            }
            #[test]
            #[ignore] // wrong result
            fn in_sub() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"c\", \"d c\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: true;\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn in_super() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"c d\", \"d\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: false;\
                     \n}\
                     \n"
                );
            }
        }
    }
    mod sibling {
        #[allow(unused)]
        use super::rsass;
        mod and_adjacent_sibling {
            #[allow(unused)]
            use super::rsass;
            mod multiple {
                #[allow(unused)]
                use super::rsass;
                #[test]
                #[ignore] // wrong result
                fn first() {
                    assert_eq!(
        rsass(
            "a {b: is-superselector(\"d ~ c\", \"d + e + c\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
                }
                #[test]
                #[ignore] // wrong result
                fn neither() {
                    assert_eq!(
        rsass(
            "a {b: is-superselector(\"f ~ c\", \"d + e + c\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
                }
                #[test]
                #[ignore] // wrong result
                fn second() {
                    assert_eq!(
        rsass(
            "a {b: is-superselector(\"e ~ c\", \"d + e + c\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
                }
            }
            #[test]
            #[ignore] // wrong result
            fn sub() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"d + c\", \"d ~ c\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: false;\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn test_super() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"d ~ c\", \"d + c\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: true;\
                     \n}\
                     \n"
                );
            }
        }
        mod multiple {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn first() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"d ~ c\", \"d ~ e ~ c\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: true;\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn in_sub() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"c\", \"d ~ e ~ c\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: true;\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn neither() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"f ~ c\", \"d ~ e ~ c\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: false;\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn second() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"e ~ c\", \"d ~ e ~ c\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: true;\
                     \n}\
                     \n"
                );
            }
        }
        mod single {
            #[allow(unused)]
            use super::rsass;
            mod in_both {
                #[allow(unused)]
                use super::rsass;
                #[test]
                #[ignore] // wrong result
                fn equal() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"c ~ d\", \"c ~ d\")}\
                             \n"
                        )
                        .unwrap(),
                        "a {\
                         \n  b: true;\
                         \n}\
                         \n"
                    );
                }
                #[test]
                #[ignore] // wrong result
                fn subset() {
                    assert_eq!(
        rsass(
            "a {b: is-superselector(\"c ~ d\", \"c.e ~ d.f\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
                }
                #[test]
                #[ignore] // wrong result
                fn superset() {
                    assert_eq!(
        rsass(
            "a {b: is-superselector(\"c.e ~ d.f\", \"c ~ d\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
                }
            }
            #[test]
            #[ignore] // wrong result
            fn in_sub() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"c\", \"d ~ c\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: true;\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn in_super() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"c ~ d\", \"d\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: false;\
                     \n}\
                     \n"
                );
            }
        }
    }
}

// From "sass-spec/spec/core_functions/selector/is_superselector/compound.hrx"
mod compound {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn different_order() {
        assert_eq!(
            rsass(
                "a {b: is-superselector(\"c.e\", \"c:d.e\")}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: true;\
             \n}\
             \n"
        );
    }
    mod pseudo_element {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn absent() {
            assert_eq!(
                rsass(
                    "a {b: is-superselector(\"c\", \"c::d\")}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: false;\
                 \n}\
                 \n"
            );
        }
        mod class_syntax {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn after() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"c\", \"c:after\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: false;\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn before() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"c\", \"c:before\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: false;\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn first_letter() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"c\", \"c:first-letter\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: false;\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn first_line() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"c\", \"c:first-line\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: false;\
                     \n}\
                     \n"
                );
            }
        }
        #[test]
        #[ignore] // wrong result
        fn different_order() {
            assert_eq!(
                rsass(
                    "a {b: is-superselector(\":e::d\", \"::d:e\")}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: false;\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn present() {
            assert_eq!(
                rsass(
                    "a {b: is-superselector(\"::d\", \"c::d\")}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: true;\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn same_order() {
            assert_eq!(
                rsass(
                    "a {b: is-superselector(\"::d:e\", \"::d:e\")}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: true;\
                 \n}\
                 \n"
            );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn same_order() {
        assert_eq!(
            rsass(
                "a {b: is-superselector(\"c\", \"c.d\")}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: true;\
             \n}\
             \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn superset() {
        assert_eq!(
            rsass(
                "a {b: is-superselector(\"c.d\", \"c\")}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: false;\
             \n}\
             \n"
        );
    }
}

// From "sass-spec/spec/core_functions/selector/is_superselector/error.hrx"
mod error {
    #[allow(unused)]
    use super::rsass;
    mod sub {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "invalid", error tests are not supported yet.

        // Ignoring "parent", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }
    mod test_super {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "invalid", error tests are not supported yet.

        // Ignoring "parent", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.
}

// From "sass-spec/spec/core_functions/selector/is_superselector/input.hrx"
#[test]
#[ignore] // wrong result
fn input() {
    assert_eq!(
        rsass(
            "// The full set of possible input formats is tested with `selector-parse()`;\
            \n// this spec just verifies one example for `is-superselector()`.\
            \na {b: is-superselector((c, d e), (c, d e))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/core_functions/selector/is_superselector/list.hrx"
mod list {
    #[allow(unused)]
    use super::rsass;
    mod three {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn match_one() {
            assert_eq!(
                rsass(
                    "a {b: is-superselector(\"c, d, e\", \"d\")}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: true;\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn match_three() {
            assert_eq!(
                rsass(
                    "a {b: is-superselector(\"c, d, e\", \"d, c, e\")}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: true;\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn match_two() {
            assert_eq!(
                rsass(
                    "a {b: is-superselector(\"c, d, e\", \"e, c\")}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: true;\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn miss_one() {
            assert_eq!(
                rsass(
                    "a {b: is-superselector(\"c, d, e\", \"c, f\")}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: false;\
                 \n}\
                 \n"
            );
        }
    }
    mod two {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn both_satisfied_by_one_superselector() {
            assert_eq!(
                rsass(
                    "a {b: is-superselector(\".c\", \"d.c, e.c\")}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: true;\
                 \n}\
                 \n"
            );
        }
        mod in_both {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn equal() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"c, d\", \"c, d\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: true;\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn subset() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"c, d\", \"c.e, d.f\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: true;\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn superset() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"c.e, d.f\", \"c, d\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: false;\
                     \n}\
                     \n"
                );
            }
        }
        #[test]
        #[ignore] // wrong result
        fn in_sub() {
            assert_eq!(
                rsass(
                    "a {b: is-superselector(\"c\", \"c, d\")}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: false;\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn in_super() {
            assert_eq!(
                rsass(
                    "a {b: is-superselector(\"c, d\", \"c\")}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: true;\
                 \n}\
                 \n"
            );
        }
    }
}

// From "sass-spec/spec/core_functions/selector/is_superselector/named.hrx"
#[test]
#[ignore] // wrong result
fn named() {
    assert_eq!(
        rsass(
            "a {b: is-superselector($super: \"c\", $sub: \"c.d\")}\
             \n"
        )
        .unwrap(),
        "a {\
         \n  b: true;\
         \n}\
         \n"
    );
}

mod simple;
