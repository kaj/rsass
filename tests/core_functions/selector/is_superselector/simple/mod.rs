//! Tests auto-converted from "sass-spec/spec/core_functions/selector/is_superselector/simple"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/core_functions/selector/is_superselector/simple/attribute.hrx"
mod attribute {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn equal() {
        assert_eq!(
            rsass(
                "a {b: is-superselector(\"[c=d]\", \"[c=d]\")}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: true;\
             \n}\
             \n"
        );
    }
    mod unequal {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn name() {
            assert_eq!(
                rsass(
                    "a {b: is-superselector(\"[c=d]\", \"[e=d]\")}\
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
        fn operator() {
            assert_eq!(
                rsass(
                    "a {b: is-superselector(\"[c=d]\", \"[c^=d]\")}\
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
        fn value() {
            assert_eq!(
                rsass(
                    "a {b: is-superselector(\"[c=d]\", \"[c=e]\")}\
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

// From "sass-spec/spec/core_functions/selector/is_superselector/simple/class.hrx"
mod class {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn equal() {
        assert_eq!(
            rsass(
                "a {b: is-superselector(\".c\", \".c\")}\
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
    fn unequal() {
        assert_eq!(
            rsass(
                "a {b: is-superselector(\".c\", \".d\")}\
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

// From "sass-spec/spec/core_functions/selector/is_superselector/simple/id.hrx"
mod id {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn equal() {
        assert_eq!(
            rsass(
                "a {b: is-superselector(\"#c\", \"#c\")}\
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
    fn unequal() {
        assert_eq!(
            rsass(
                "a {b: is-superselector(\"#c\", \"#d\")}\
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

// From "sass-spec/spec/core_functions/selector/is_superselector/simple/placeholder.hrx"
mod placeholder {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn equal() {
        assert_eq!(
            rsass(
                "a {b: is-superselector(\"%c\", \"%c\")}\
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
    fn unequal() {
        assert_eq!(
            rsass(
                "a {b: is-superselector(\"%c\", \"%d\")}\
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

mod pseudo;

// From "sass-spec/spec/core_functions/selector/is_superselector/simple/type.hrx"
mod test_type {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn and_universal() {
        assert_eq!(
            rsass(
                "a {b: is-superselector(\"c\", \"*\")}\
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
    fn equal() {
        assert_eq!(
            rsass(
                "a {b: is-superselector(\"c\", \"c\")}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: true;\
             \n}\
             \n"
        );
    }
    mod namespace {
        #[allow(unused)]
        use super::rsass;
        mod empty {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn and_empty() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"|c\", \"|c\")}\
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
            fn and_explicit() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"|c\", \"d|c\")}\
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
            fn and_implicit() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"|c\", \"c\")}\
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
            fn and_universal() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"|c\", \"*|c\")}\
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
        mod explicit {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn and_empty() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"c|d\", \"|d\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: false;\
                     \n}\
                     \n"
                );
            }
            mod and_explicit {
                #[allow(unused)]
                use super::rsass;
                #[test]
                #[ignore] // wrong result
                fn equal() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"c|d\", \"c|d\")}\
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
                fn unequal() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"c|d\", \"e|d\")}\
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
            fn and_implicit() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"c|d\", \"d\")}\
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
            fn and_universal() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"c|d\", \"*|d\")}\
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
        mod universal {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn and_empty() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"*|c\", \"|c\")}\
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
            fn and_explicit() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"*|c\", \"d|c\")}\
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
            fn and_implicit() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"*|c\", \"c\")}\
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
            fn and_universal() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"*|c\", \"*|c\")}\
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
    #[test]
    #[ignore] // wrong result
    fn unequal() {
        assert_eq!(
            rsass(
                "a {b: is-superselector(\"c\", \"d\")}\
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

// From "sass-spec/spec/core_functions/selector/is_superselector/simple/universal.hrx"
mod universal {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn and_class() {
        assert_eq!(
            rsass(
                "a {b: is-superselector(\"*\", \".c\")}\
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
    fn and_type() {
        assert_eq!(
            rsass(
                "a {b: is-superselector(\"*\", \"c\")}\
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
    fn equal() {
        assert_eq!(
            rsass(
                "a {b: is-superselector(\"*\", \"*\")}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: true;\
             \n}\
             \n"
        );
    }
    mod namespace {
        #[allow(unused)]
        use super::rsass;
        mod empty {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn and_class() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"|*\", \".d\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: false;\
                     \n}\
                     \n"
                );
            }
            mod and_type {
                #[allow(unused)]
                use super::rsass;
                #[test]
                #[ignore] // wrong result
                fn empty() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"|*\", \"|d\")}\
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
                fn explicit() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"|*\", \"c|d\")}\
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
                fn implicit() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"|*\", \"d\")}\
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
            mod and_universal {
                #[allow(unused)]
                use super::rsass;
                #[test]
                #[ignore] // wrong result
                fn empty() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"|*\", \"|*\")}\
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
                fn explicit() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"|*\", \"c|*\")}\
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
                fn implicit() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"|*\", \"*\")}\
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
                fn universal() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"|*\", \"*|*\")}\
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
        mod explicit {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn and_class() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"c|*\", \".d\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: false;\
                     \n}\
                     \n"
                );
            }
            mod and_type {
                #[allow(unused)]
                use super::rsass;
                #[test]
                #[ignore] // wrong result
                fn empty() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"c|*\", \"|d\")}\
                             \n"
                        )
                        .unwrap(),
                        "a {\
                         \n  b: false;\
                         \n}\
                         \n"
                    );
                }
                mod explicit {
                    #[allow(unused)]
                    use super::rsass;
                    #[test]
                    #[ignore] // wrong result
                    fn equal() {
                        assert_eq!(
                            rsass(
                                "a {b: is-superselector(\"c|*\", \"c|d\")}\
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
                    fn unequal() {
                        assert_eq!(
                            rsass(
                                "a {b: is-superselector(\"c|*\", \"e|d\")}\
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
                fn implicit() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"c|*\", \"d\")}\
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
            mod and_universal {
                #[allow(unused)]
                use super::rsass;
                #[test]
                #[ignore] // wrong result
                fn empty() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"c|*\", \"|*\")}\
                             \n"
                        )
                        .unwrap(),
                        "a {\
                         \n  b: false;\
                         \n}\
                         \n"
                    );
                }
                mod explicit {
                    #[allow(unused)]
                    use super::rsass;
                    #[test]
                    #[ignore] // wrong result
                    fn equal() {
                        assert_eq!(
                            rsass(
                                "a {b: is-superselector(\"c|*\", \"c|*\")}\
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
                    fn unequal() {
                        assert_eq!(
                            rsass(
                                "a {b: is-superselector(\"c|*\", \"d|*\")}\
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
                fn implicit() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"c|*\", \"*\")}\
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
                fn universal() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"c|*\", \"*|*\")}\
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
        mod universal {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn and_class() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"*|*\", \".d\")}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: true;\
                     \n}\
                     \n"
                );
            }
            mod and_type {
                #[allow(unused)]
                use super::rsass;
                #[test]
                #[ignore] // wrong result
                fn empty() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"*|*\", \"|d\")}\
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
                fn explicit() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"*|*\", \"c|d\")}\
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
                fn implicit() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"*|*\", \"d\")}\
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
            mod and_universal {
                #[allow(unused)]
                use super::rsass;
                #[test]
                #[ignore] // wrong result
                fn empty() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"*|*\", \"|*\")}\
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
                fn explicit() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"*|*\", \"c|*\")}\
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
                fn implicit() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"*|*\", \"*\")}\
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
                fn universal() {
                    assert_eq!(
                        rsass(
                            "a {b: is-superselector(\"*|*\", \"*|*\")}\
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
    }
}
