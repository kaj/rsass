//! Tests auto-converted from "sass-spec/spec/core_functions/meta/inspect.hrx"

mod boolean {
    #[test]
    fn test_false() {
        assert_eq!(
            crate::rsass(
                "$result: inspect(false);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  value: false;\
        \n  type: string;\
        \n}\
        \n"
        );
    }
    #[test]
    fn test_true() {
        assert_eq!(
            crate::rsass(
                "$result: inspect(true);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  value: true;\
        \n  type: string;\
        \n}\
        \n"
        );
    }
}
mod color {
    mod generated {
        #[test]
        fn alpha() {
            assert_eq!(
                crate::rsass(
                    "$result: inspect(rgba(1, 2, 3, 0.4));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  value: rgba(1, 2, 3, 0.4);\
        \n  type: string;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn long_hex() {
            assert_eq!(
                crate::rsass(
                    "@import \"../utils\";\
            \n$result: inspect(generated-color(#abcdef));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  value: #abcdef;\
        \n  type: string;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn named() {
            assert_eq!(
                crate::rsass(
                    "@import \"../utils\";\
            \n$result: inspect(generated-color(#00f));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  value: blue;\
        \n  type: string;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn short_hex() {
            assert_eq!(
                crate::rsass(
                    "@import \"../utils\";\
            \n$result: inspect(generated-color(#abc));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  value: #aabbcc;\
        \n  type: string;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn transparent() {
            assert_eq!(
                crate::rsass(
                    "@import \"../utils\";\
            \n$result: inspect(generated-color(transparent));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  value: rgba(0, 0, 0, 0);\
        \n  type: string;\
        \n}\
        \n"
            );
        }
    }
    mod literal {
        #[test]
        fn long_hex() {
            assert_eq!(
                crate::rsass(
                    "$result: inspect(#0000ff);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  value: #0000ff;\
        \n  type: string;\
        \n}\
        \n"
            );
        }
        #[test]
        fn named() {
            assert_eq!(
                crate::rsass(
                    "$result: inspect(blue);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  value: blue;\
        \n  type: string;\
        \n}\
        \n"
            );
        }
        #[test]
        fn short_hex() {
            assert_eq!(
                crate::rsass(
                    "$result: inspect(#00f);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  value: #00f;\
        \n  type: string;\
        \n}\
        \n"
            );
        }
        #[test]
        fn transparent() {
            assert_eq!(
                crate::rsass(
                    "$result: inspect(transparent);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  value: transparent;\
        \n  type: string;\
        \n}\
        \n"
            );
        }
    }
}
mod error {

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.
}
#[test]
fn function() {
    assert_eq!(
        crate::rsass(
            "$result: inspect(get-function(\"get-function\"));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  value: get-function(\"get-function\");\
        \n  type: string;\
        \n}\
        \n"
    );
}
mod list {
    #[test]
    fn bracketed() {
        assert_eq!(
            crate::rsass(
                "$result: inspect([1, 2, 3]);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  value: [1, 2, 3];\
        \n  type: string;\
        \n}\
        \n"
        );
    }
    #[test]
    fn comma() {
        assert_eq!(
            crate::rsass(
                "$result: inspect((1, 2, 3));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  value: 1, 2, 3;\
        \n  type: string;\
        \n}\
        \n"
        );
    }
    #[test]
    fn empty() {
        assert_eq!(
            crate::rsass(
                "$result: inspect(());\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  value: ();\
        \n  type: string;\
        \n}\
        \n"
        );
    }
    mod nested {
        mod bracketed {
            mod in_comma {
                #[test]
                fn bracketed() {
                    assert_eq!(
                        crate::rsass(
                            "$result: inspect([[1, 2], [3, 4]]);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  value: [[1, 2], [3, 4]];\
        \n  type: string;\
        \n}\
        \n"
                    );
                }
                #[test]
                fn unbracketed() {
                    assert_eq!(
                        crate::rsass(
                            "$result: inspect(((1, 2), (3, 4)));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  value: (1, 2), (3, 4);\
        \n  type: string;\
        \n}\
        \n"
                    );
                }
            }
            mod in_space {
                #[test]
                fn bracketed() {
                    assert_eq!(
                        crate::rsass(
                            "$result: inspect([[1, 2] [3, 4]]);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  value: [[1, 2] [3, 4]];\
        \n  type: string;\
        \n}\
        \n"
                    );
                }
                #[test]
                fn unbracketed() {
                    assert_eq!(
                        crate::rsass(
                            "$result: inspect([1, 2] [3, 4]);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  value: [1, 2] [3, 4];\
        \n  type: string;\
        \n}\
        \n"
                    );
                }
            }
        }
        mod comma {
            mod in_comma {
                #[test]
                fn bracketed() {
                    assert_eq!(
                        crate::rsass(
                            "$result: inspect([(1, 2), (3, 4)]);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  value: [(1, 2), (3, 4)];\
        \n  type: string;\
        \n}\
        \n"
                    );
                }
                #[test]
                fn unbracketed() {
                    assert_eq!(
                        crate::rsass(
                            "$result: inspect(((1, 2), (3, 4)));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  value: (1, 2), (3, 4);\
        \n  type: string;\
        \n}\
        \n"
                    );
                }
            }
            mod in_space {
                #[test]
                fn bracketed() {
                    assert_eq!(
                        crate::rsass(
                            "$result: inspect([(1, 2) (3, 4)]);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  value: [(1, 2) (3, 4)];\
        \n  type: string;\
        \n}\
        \n"
                    );
                }
                #[test]
                fn unbracketed() {
                    assert_eq!(
                        crate::rsass(
                            "$result: inspect((1, 2) (3, 4));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  value: (1, 2) (3, 4);\
        \n  type: string;\
        \n}\
        \n"
                    );
                }
            }
        }
        mod empty {
            mod in_comma {
                #[test]
                fn bracketed() {
                    assert_eq!(
                        crate::rsass(
                            "$result: inspect([(), ()]);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  value: [(), ()];\
        \n  type: string;\
        \n}\
        \n"
                    );
                }
                #[test]
                fn unbracketed() {
                    assert_eq!(
                        crate::rsass(
                            "$result: inspect(((), ()));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  value: (), ();\
        \n  type: string;\
        \n}\
        \n"
                    );
                }
            }
            mod in_space {
                #[test]
                fn bracketed() {
                    assert_eq!(
                        crate::rsass(
                            "$result: inspect([() ()]);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  value: [() ()];\
        \n  type: string;\
        \n}\
        \n"
                    );
                }
                #[test]
                fn unbracketed() {
                    assert_eq!(
                        crate::rsass(
                            "$result: inspect(() ());\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  value: () ();\
        \n  type: string;\
        \n}\
        \n"
                    );
                }
            }
        }
        mod space {
            mod in_comma {
                #[test]
                fn bracketed() {
                    assert_eq!(
                        crate::rsass(
                            "$result: inspect([1 2, 3 4]);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  value: [1 2, 3 4];\
        \n  type: string;\
        \n}\
        \n"
                    );
                }
                #[test]
                fn unbracketed() {
                    assert_eq!(
                        crate::rsass(
                            "$result: inspect((1 2, 3 4));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  value: 1 2, 3 4;\
        \n  type: string;\
        \n}\
        \n"
                    );
                }
            }
            mod in_space {
                #[test]
                fn bracketed() {
                    assert_eq!(
                        crate::rsass(
                            "$result: inspect([(1 2) (3 4)]);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  value: [(1 2) (3 4)];\
        \n  type: string;\
        \n}\
        \n"
                    );
                }
                #[test]
                fn unbracketed() {
                    assert_eq!(
                        crate::rsass(
                            "$result: inspect((1 2) (3 4));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  value: (1 2) (3 4);\
        \n  type: string;\
        \n}\
        \n"
                    );
                }
            }
        }
    }
    mod single {
        mod bracketed {
            #[test]
            fn comma() {
                assert_eq!(
                    crate::rsass(
                        "$result: inspect([1,]);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  value: [1,];\
        \n  type: string;\
        \n}\
        \n"
                );
            }
            #[test]
            fn undecided() {
                assert_eq!(
                    crate::rsass(
                        "$result: inspect([1]);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  value: [1];\
        \n  type: string;\
        \n}\
        \n"
                );
            }
        }
        #[test]
        fn comma() {
            assert_eq!(
                crate::rsass(
                    "$result: inspect((1,));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  value: (1,);\
        \n  type: string;\
        \n}\
        \n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
                crate::rsass(
                    "$result: inspect(append((), 1, space));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  value: 1;\
        \n  type: string;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn space() {
        assert_eq!(
            crate::rsass(
                "$result: inspect(1 2 3);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  value: 1 2 3;\
        \n  type: string;\
        \n}\
        \n"
        );
    }
}
mod map {
    mod list {
        mod key {
            #[test]
            fn comma() {
                assert_eq!(
                    crate::rsass(
                        "$result: inspect(((1, 2): 3, (4, 5): 6));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  value: ((1, 2): 3, (4, 5): 6);\
        \n  type: string;\
        \n}\
        \n"
                );
            }
            #[test]
            fn space() {
                assert_eq!(
                    crate::rsass(
                        "$result: inspect((1 2: 3, 4 5: 6));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  value: (1 2: 3, 4 5: 6);\
        \n  type: string;\
        \n}\
        \n"
                );
            }
        }
        mod value {
            #[test]
            fn comma() {
                assert_eq!(
                    crate::rsass(
                        "$result: inspect((1: (2, 3), 4: (5, 6)));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  value: (1: (2, 3), 4: (5, 6));\
        \n  type: string;\
        \n}\
        \n"
                );
            }
            #[test]
            fn space() {
                assert_eq!(
                    crate::rsass(
                        "$result: inspect((1: 2 3, 4: 5 6));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  value: (1: 2 3, 4: 5 6);\
        \n  type: string;\
        \n}\
        \n"
                );
            }
        }
    }
    #[test]
    fn number() {
        assert_eq!(
            crate::rsass(
                "$result: inspect((1: 2, 3: 4));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  value: (1: 2, 3: 4);\
        \n  type: string;\
        \n}\
        \n"
        );
    }
}
#[test]
fn null() {
    assert_eq!(
        crate::rsass(
            "$result: inspect(null);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  value: null;\
        \n  type: string;\
        \n}\
        \n"
    );
}
mod number {
    #[test]
    fn unit() {
        assert_eq!(
        crate::rsass(
            "// We explicitly don\'t test the inspect format for complex units. Their format\
            \n// isn\'t guaranteed by the spec, since they can\'t be written literally in Sass.\
            \n$result: inspect(50px);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  value: 50px;\
        \n  type: string;\
        \n}\
        \n"
    );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            crate::rsass(
                "$result: inspect(123.456);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  value: 123.456;\
        \n  type: string;\
        \n}\
        \n"
        );
    }
}
mod string {
    #[test]
    fn quoted() {
        assert_eq!(
        crate::rsass(
            "$result: inspect(\"foo\");\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n\
            \n  // inspect() should always return an unquoted string, so when it\'s passed a\
            \n  // quoted string its return value should contain quote characters. We check\
            \n  // the length to verify that the quotes are included, since there\'s no\
            \n  // built-in way to check whether a string is quoted.\
            \n  length: str-length($result);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  value: \"foo\";\
        \n  type: string;\
        \n  length: 5;\
        \n}\
        \n"
    );
    }
    #[test]
    fn unquoted() {
        assert_eq!(
            crate::rsass(
                "$result: inspect(foo);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  value: foo;\
        \n  type: string;\
        \n}\
        \n"
        );
    }
}
