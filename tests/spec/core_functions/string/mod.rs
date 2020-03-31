//! Tests auto-converted from "sass-spec/spec/core_functions/string"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/core_functions/string/index.hrx"
mod index {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn beginning() {
        assert_eq!(
            rsass(
                "a {b: str-index(\"cde\", \"c\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1;\
        \n}\
        \n"
        );
    }
    #[test]
    fn both_empty() {
        assert_eq!(
            rsass(
                "a {b: str-index(\"\", \"\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1;\
        \n}\
        \n"
        );
    }
    #[test]
    fn empty_substring() {
        assert_eq!(
            rsass(
                "a {b: str-index(\"cde\", \"\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1;\
        \n}\
        \n"
        );
    }
    #[test]
    fn end() {
        assert_eq!(
            rsass(
                "a {b: str-index(\"cde\", \"e\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 3;\
        \n}\
        \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.
        mod test_type {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "string", error tests are not supported yet.

            // Ignoring "substring", error tests are not supported yet.
        }
    }
    #[test]
    fn middle() {
        assert_eq!(
            rsass(
                "a {b: str-index(\"cde\", \"d\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 2;\
        \n}\
        \n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: str-index($string: \"cde\", $substring: \"c\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1;\
        \n}\
        \n"
        );
    }
    #[test]
    fn not_found() {
        assert_eq!(
            rsass(
                "a {b: inspect(str-index(\"cde\", \"f\"))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: null;\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/string/insert.hrx"
mod insert {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn combining_character() {
        assert_eq!(
        rsass(
            "// Sass does *not* treat strings as sequences of glyphs, so this string which\
            \n// contains \"c\" followed by a combining umlaut should be considered two separate\
            \n// characters even though it\'s rendered as only one and the \"d\" should be\
            \n// injected between the two.\
            \na {b: str-insert(\"c\\0308\", \"d\", 2)}\
            \n"
        )
        .unwrap(),
        "@charset \"UTF-8\";\
        \na {\
        \n  b: \"cd\u{308}\";\
        \n}\
        \n"
    );
    }
    #[test]
    fn double_width_character() {
        assert_eq!(
        rsass(
            "// Sass treats strings as sequences of Unicode codepoint; it doesn\'t care if a\
            \n// character is represented as two UTF-16 code units, so inserting a character\
            \n// at index 2 shouldn\'t break this emoji in two.\
            \na {b: str-insert(\"👭\", \"c\", 2)}\
            \n"
        )
        .unwrap(),
        "@charset \"UTF-8\";\
        \na {\
        \n  b: \"👭c\";\
        \n}\
        \n"
    );
    }
    mod empty_destination {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn empty_source() {
            assert_eq!(
                rsass(
                    "a {b: str-insert(\"\", \"\", 1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: \"\";\
        \n}\
        \n"
            );
        }
        #[test]
        fn index_0() {
            assert_eq!(
                rsass(
                    "a {b: str-insert(\"\", \"c\", 0)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: \"c\";\
        \n}\
        \n"
            );
        }
        #[test]
        fn index_1() {
            assert_eq!(
                rsass(
                    "a {b: str-insert(\"\", \"c\", 1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: \"c\";\
        \n}\
        \n"
            );
        }
        #[test]
        fn index_2() {
            assert_eq!(
                rsass(
                    "a {b: str-insert(\"\", \"c\", 2)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: \"c\";\
        \n}\
        \n"
            );
        }
        #[test]
        fn index_negative_1() {
            assert_eq!(
                rsass(
                    "a {b: str-insert(\"\", \"c\", -1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: \"c\";\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn empty_insertion() {
        assert_eq!(
            rsass(
                "a {b: str-insert(\"cde\", \"\", 1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: \"cde\";\
        \n}\
        \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "decimal", error tests are not supported yet.

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.
        mod test_type {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "index", error tests are not supported yet.

            // Ignoring "insert", error tests are not supported yet.

            // Ignoring "string", error tests are not supported yet.
        }
    }
    mod index {
        #[allow(unused)]
        use super::rsass;
        mod negative {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn t1() {
                assert_eq!(
                    rsass(
                        "a {b: str-insert(\"cde\", \"f\", -1)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"cdef\";\
        \n}\
        \n"
                );
            }
            #[test]
            fn t2() {
                assert_eq!(
                    rsass(
                        "a {b: str-insert(\"cde\", \"f\", -2)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"cdfe\";\
        \n}\
        \n"
                );
            }
            #[test]
            fn after_last() {
                assert_eq!(
                    rsass(
                        "a {b: str-insert(\"cde\", \"f\", -100)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"fcde\";\
        \n}\
        \n"
                );
            }
            #[test]
            fn last() {
                assert_eq!(
                    rsass(
                        "a {b: str-insert(\"cde\", \"f\", -4)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"fcde\";\
        \n}\
        \n"
                );
            }
        }
        mod positive {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn t0() {
                assert_eq!(
                    rsass(
                        "a {b: str-insert(\"cde\", \"f\", 0)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"fcde\";\
        \n}\
        \n"
                );
            }
            #[test]
            fn t1() {
                assert_eq!(
                    rsass(
                        "a {b: str-insert(\"cde\", \"f\", 1)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"fcde\";\
        \n}\
        \n"
                );
            }
            #[test]
            fn t2() {
                assert_eq!(
                    rsass(
                        "a {b: str-insert(\"cde\", \"f\", 2)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"cfde\";\
        \n}\
        \n"
                );
            }
            #[test]
            fn after_last() {
                assert_eq!(
                    rsass(
                        "a {b: str-insert(\"cde\", \"f\", 100)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"cdef\";\
        \n}\
        \n"
                );
            }
            #[test]
            fn last() {
                assert_eq!(
                    rsass(
                        "a {b: str-insert(\"cde\", \"f\", 4)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"cdef\";\
        \n}\
        \n"
                );
            }
        }
    }
    #[test]
    fn named() {
        assert_eq!(
        rsass(
            "a {b: str-insert($string: \"cde\", $insert: \"f\", $index: 2)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"cfde\";\
        \n}\
        \n"
    );
    }
}

// From "sass-spec/spec/core_functions/string/length.hrx"
mod length {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn combining_character() {
        assert_eq!(
        rsass(
            "// Sass does *not* treat strings as sequences of glyphs, so this string which\
            \n// contains \"c\" followed by a combining umlaut should be considered two separate\
            \n// characters even though it\'s rendered as only one.\
            \na {b: str-length(\"c\\0308\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 2;\
        \n}\
        \n"
    );
    }
    #[test]
    fn double_width_character() {
        assert_eq!(
        rsass(
            "// Sass treats strings as sequences of Unicode codepoint; it doesn\'t care if a\
            \n// character is represented as two UTF-16 code units.\
            \na {b: str-length(\"👭\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
    }
    #[test]
    fn empty() {
        assert_eq!(
            rsass(
                "a {b: str-length(\"\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0;\
        \n}\
        \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }
    #[test]
    fn multiple_characters() {
        assert_eq!(
            rsass(
                "a {b: str-length(\"fblthp abatement\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 16;\
        \n}\
        \n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: str-length($string: \"c\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1;\
        \n}\
        \n"
        );
    }
    #[test]
    fn one_character() {
        assert_eq!(
            rsass(
                "a {b: str-length(\"c\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1;\
        \n}\
        \n"
        );
    }
    #[test]
    fn unquoted() {
        assert_eq!(
            rsass(
                "a {b: str-length(loofamonster)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 12;\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/string/quote.hrx"
mod quote {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }
    #[test]
    fn escape() {
        assert_eq!(
            rsass(
                "a {b: quote(\\0)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: \"\\\\0 \";\
        \n}\
        \n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: quote($string: c)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: \"c\";\
        \n}\
        \n"
        );
    }
    mod quote_unquoted_quote {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn double() {
            assert_eq!(
                rsass(
                    "// See sass/libsass#2873\
            \na {b: quote(unquote(\'\"\') + unquote(\"\'\"))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: \"\\\"\'\";\
        \n}\
        \n"
            );
        }
        #[test]
        fn single() {
            assert_eq!(
                rsass(
                    "// See sass/libsass#2873\
            \na {b: quote(unquote(\'\"\'))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: \'\"\';\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn quoted_double() {
        assert_eq!(
            rsass(
                "a {b: quote(\"c\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: \"c\";\
        \n}\
        \n"
        );
    }
    #[test]
    fn quoted_single() {
        assert_eq!(
            rsass(
                "a {b: quote(\'c\')}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: \"c\";\
        \n}\
        \n"
        );
    }
    #[test]
    fn unquoted() {
        assert_eq!(
            rsass(
                "a {b: quote(c)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: \"c\";\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/string/slice.hrx"
mod slice {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn combining_character() {
        assert_eq!(
        rsass(
            "// Sass does *not* treat strings as sequences of glyphs, so this string which\
            \n// contains \"c\" followed by a combining umlaut should be considered two separate\
            \n// characters even though it\'s rendered as only one and only the \"d\" should be\
            \n// sliced out.\
            \na {b: str-slice(\"cd\\0308e\", 2, 2)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"d\";\
        \n}\
        \n"
    );
    }
    #[test]
    fn double_width_character() {
        assert_eq!(
        rsass(
            "// Sass treats strings as sequences of Unicode codepoint; it doesn\'t care if a\
            \n// character is represented as two UTF-16 code units, so inserting a character\
            \n// at index 2 shouldn\'t break this emoji in two.\
            \na {b: str-slice(\"c👭d\", 2, 2)}\
            \n"
        )
        .unwrap(),
        "@charset \"UTF-8\";\
        \na {\
        \n  b: \"👭\";\
        \n}\
        \n"
    );
    }
    mod empty {
        #[allow(unused)]
        use super::rsass;
        mod end {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn t0() {
                assert_eq!(
                    rsass(
                        "a {b: str-slice(\"\", 1, 0)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"\";\
        \n}\
        \n"
                );
            }
            #[test]
            fn t1() {
                assert_eq!(
                    rsass(
                        "a {b: str-slice(\"\", 1, 1)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"\";\
        \n}\
        \n"
                );
            }
            #[test]
            fn t2() {
                assert_eq!(
                    rsass(
                        "a {b: str-slice(\"\", 1, 2)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"\";\
        \n}\
        \n"
                );
            }
        }
        mod start {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn t0() {
                assert_eq!(
                    rsass(
                        "a {b: str-slice(\"\", 0)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"\";\
        \n}\
        \n"
                );
            }
            #[test]
            fn t1() {
                assert_eq!(
                    rsass(
                        "a {b: str-slice(\"\", 1)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"\";\
        \n}\
        \n"
                );
            }
            #[test]
            fn t2() {
                assert_eq!(
                    rsass(
                        "a {b: str-slice(\"\", 2)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"\";\
        \n}\
        \n"
                );
            }
            #[test]
            fn negative_1() {
                assert_eq!(
                    rsass(
                        "a {b: str-slice(\"\", -1)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"\";\
        \n}\
        \n"
                );
            }
        }
    }
    mod end {
        #[allow(unused)]
        use super::rsass;
        mod negative {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn t1() {
                assert_eq!(
                    rsass(
                        "a {b: str-slice(\"cde\", 1, -1)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"cde\";\
        \n}\
        \n"
                );
            }
            #[test]
            fn t2() {
                assert_eq!(
                    rsass(
                        "a {b: str-slice(\"cde\", 1, -2)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"cd\";\
        \n}\
        \n"
                );
            }
            #[test]
            fn after_last() {
                assert_eq!(
                    rsass(
                        "a {b: str-slice(\"cde\", 1, -100)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"\";\
        \n}\
        \n"
                );
            }
            #[test]
            fn last() {
                assert_eq!(
                    rsass(
                        "a {b: str-slice(\"cde\", 1, -4)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"\";\
        \n}\
        \n"
                );
            }
        }
        mod positive {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn t0() {
                assert_eq!(
                    rsass(
                        "a {b: str-slice(\"cde\", 1, 0)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"\";\
        \n}\
        \n"
                );
            }
            #[test]
            fn t1() {
                assert_eq!(
                    rsass(
                        "a {b: str-slice(\"cde\", 1, 1)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"c\";\
        \n}\
        \n"
                );
            }
            #[test]
            fn t2() {
                assert_eq!(
                    rsass(
                        "a {b: str-slice(\"cde\", 1, 2)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"cd\";\
        \n}\
        \n"
                );
            }
            #[test]
            fn after_last() {
                assert_eq!(
                    rsass(
                        "a {b: str-slice(\"cde\", 1, 100)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"cde\";\
        \n}\
        \n"
                );
            }
            #[test]
            fn after_start() {
                assert_eq!(
                    rsass(
                        "a {b: str-slice(\"cdef\", 2, 3)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"de\";\
        \n}\
        \n"
                );
            }
            #[test]
            fn last() {
                assert_eq!(
                    rsass(
                        "a {b: str-slice(\"cde\", 1, 3)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"cde\";\
        \n}\
        \n"
                );
            }
        }
    }
    mod error {
        #[allow(unused)]
        use super::rsass;
        mod decimal {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "end", error tests are not supported yet.

            // Ignoring "start", error tests are not supported yet.
        }

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.
        mod test_type {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "end_at", error tests are not supported yet.

            // Ignoring "start_at", error tests are not supported yet.

            // Ignoring "string", error tests are not supported yet.
        }
    }
    #[test]
    fn named() {
        assert_eq!(
        rsass(
            "a {b: str-slice($string: \"cde\", $start-at: 2, $end-at: 2)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"d\";\
        \n}\
        \n"
    );
    }
    mod start {
        #[allow(unused)]
        use super::rsass;
        mod negative {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn t1() {
                assert_eq!(
                    rsass(
                        "a {b: str-slice(\"cde\", -1)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"e\";\
        \n}\
        \n"
                );
            }
            #[test]
            fn t2() {
                assert_eq!(
                    rsass(
                        "a {b: str-slice(\"cde\", -2)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"de\";\
        \n}\
        \n"
                );
            }
            #[test]
            fn after_last() {
                assert_eq!(
                    rsass(
                        "a {b: str-slice(\"cde\", -100)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"cde\";\
        \n}\
        \n"
                );
            }
            #[test]
            fn last() {
                assert_eq!(
                    rsass(
                        "a {b: str-slice(\"cde\", -3)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"cde\";\
        \n}\
        \n"
                );
            }
        }
        mod positive {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn t0() {
                assert_eq!(
                    rsass(
                        "a {b: str-slice(\"cde\", 0)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"cde\";\
        \n}\
        \n"
                );
            }
            #[test]
            fn t1() {
                assert_eq!(
                    rsass(
                        "a {b: str-slice(\"cde\", 1)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"cde\";\
        \n}\
        \n"
                );
            }
            #[test]
            fn t2() {
                assert_eq!(
                    rsass(
                        "a {b: str-slice(\"cde\", 2)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"de\";\
        \n}\
        \n"
                );
            }
            #[test]
            fn after_end() {
                assert_eq!(
                    rsass(
                        "a {b: str-slice(\"cdef\", 3, 2)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"\";\
        \n}\
        \n"
                );
            }
            #[test]
            fn after_last() {
                assert_eq!(
                    rsass(
                        "a {b: str-slice(\"cde\", 100)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"\";\
        \n}\
        \n"
                );
            }
            #[test]
            fn last() {
                assert_eq!(
                    rsass(
                        "a {b: str-slice(\"cde\", 4)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: \"\";\
        \n}\
        \n"
                );
            }
        }
    }
    #[test]
    fn unquoted() {
        assert_eq!(
            rsass(
                "a {b: str-slice(cdefgh, 3, 5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: efg;\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/string/to_lower_case.hrx"
mod to_lower_case {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn alphabet() {
        assert_eq!(
            rsass(
                "a {b: to-lower-case(\"ABCDEFGHIJKLMNOPQRSTUVQXYZ\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: \"abcdefghijklmnopqrstuvqxyz\";\
        \n}\
        \n"
        );
    }
    #[test]
    fn empty() {
        assert_eq!(
            rsass(
                "a {b: to-lower-case(\"\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: \"\";\
        \n}\
        \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: to-lower-case($string: abcDEF)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: abcdef;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn non_ascii() {
        assert_eq!(
            rsass(
                "// Only ASCII characters have their case changed.\
            \na {b: to-lower-case(\"ÄÇÐØÞ\")}\
            \n"
            )
            .unwrap(),
            "@charset \"UTF-8\";\
        \na {\
        \n  b: \"ÄÇÐØÞ\";\
        \n}\
        \n"
        );
    }
    #[test]
    fn number() {
        assert_eq!(
            rsass(
                "a {b: to-lower-case(\"1234567890\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: \"1234567890\";\
        \n}\
        \n"
        );
    }
    #[test]
    fn unquoted() {
        assert_eq!(
            rsass(
                "a {b: to-lower-case(aBcDeF)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: abcdef;\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/string/to_upper_case.hrx"
mod to_upper_case {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn alphabet() {
        assert_eq!(
            rsass(
                "a {b: to-upper-case(\"abcdefghijklmnopqrstuvqxyz\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: \"ABCDEFGHIJKLMNOPQRSTUVQXYZ\";\
        \n}\
        \n"
        );
    }
    #[test]
    fn empty() {
        assert_eq!(
            rsass(
                "a {b: to-upper-case(\"\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: \"\";\
        \n}\
        \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: to-upper-case($string: abcDEF)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: ABCDEF;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn non_ascii() {
        assert_eq!(
            rsass(
                "// Only ASCII characters have their case changed.\
            \na {b: to-upper-case(\"äçðøþ\")}\
            \n"
            )
            .unwrap(),
            "@charset \"UTF-8\";\
        \na {\
        \n  b: \"äçðøþ\";\
        \n}\
        \n"
        );
    }
    #[test]
    fn number() {
        assert_eq!(
            rsass(
                "a {b: to-upper-case(\"1234567890\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: \"1234567890\";\
        \n}\
        \n"
        );
    }
    #[test]
    fn unquoted() {
        assert_eq!(
            rsass(
                "a {b: to-upper-case(aBcDeF)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: ABCDEF;\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/string/unique_id.hrx"
mod unique_id {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_many_args", error tests are not supported yet.
    }
    #[test]
    fn is_identifier() {
        assert_eq!(
        rsass(
            "// Every call to unique-id() should return a valid CSS identifier. We can\'t test\
            \n// this directly, so we make sure it can parse as a class selector with\
            \n// selector-parse().\
            \n@for $i from 1 to 1000 {\
            \n  $_: selector-parse(\".#{unique-id()}\");\
            \n}\
            \n"
        )
        .unwrap(),
        ""
    );
    }
    #[test]
    fn is_unique() {
        assert_eq!(
        rsass(
            "// As the name suggests, every call to unique-id() should return a different\
            \n// value.\
            \n$ids: ();\
            \n@for $i from 1 to 1000 {\
            \n  $id: unique-id();\
            \n  @if map-has-key($ids, $id) {\
            \n    @error \"#{$id} generated more than once\";\
            \n  }\
            \n\
            \n  $ids: map-merge($ids, ($id: null));\
            \n}\
            \n"
        )
        .unwrap(),
        ""
    );
    }
}

// From "sass-spec/spec/core_functions/string/unquote.hrx"
mod unquote {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn empty() {
        assert_eq!(
        rsass(
            "$result: unquote(\"\");\
            \na {\
            \n  result: $result; // This will not be emitted because the contents is empty.\
            \n  length: str-length($result);\
            \n  same: $result == \"\";\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  length: 0;\
        \n  same: true;\
        \n}\
        \n"
    );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }
    #[test]
    fn escaped_backslash() {
        assert_eq!(
            rsass(
                "$result: unquote(\"\\\\0 \");\
            \na {\
            \n  result: $result;\
            \n  length: str-length($result);\
            \n  same-as-argument: $result == \"\\\\0 \";\
            \n  same-as-literal: $result == \\0 ;\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  result: \\0 ;\
        \n  length: 3;\
        \n  same-as-argument: true;\
        \n  same-as-literal: true;\
        \n}\
        \n"
        );
    }
    mod escaped_quotes {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn quoted() {
            assert_eq!(
        rsass(
            "// Unquoting a quoted string returns an unquoted string with the same code\
            \n// points. Code points such as quotes that need to be escaped in the original\
            \n// don\'t need escaping in the output.\
            \n$result: unquote(\"\\\"c\\\"\");\
            \na {\
            \n  result: $result;\
            \n  length: str-length($result);\
            \n  same: $result == \"\\\"c\\\"\";\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  result: \"c\";\
        \n  length: 3;\
        \n  same: true;\
        \n}\
        \n"
    );
        }
        #[test]
        fn unquoted() {
            assert_eq!(
        rsass(
            "// Unquoting an unquoted string returns it exactly as-is, leaving escapes\
            \n// totally unchanged (whether they\'re quotes or not).\
            \n$result: unquote(\\\"c\\\");\
            \na {\
            \n  result: $result;\
            \n  length: str-length($result);\
            \n  same: $result == \\\"c\\\";\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  result: \\\"c\\\";\
        \n  length: 5;\
        \n  same: true;\
        \n}\
        \n"
    );
        }
    }
    #[test]
    fn meaningful_css_characters() {
        assert_eq!(
        rsass(
            "// Unquoted strings aren\'t required to be valid CSS identifiers, and the\
            \n// `unquote()` function does *not* escape characters that aren\'t valid\
            \n// identifier characters. This allows it to be used as an escape hatch to\
            \n// produce CSS that Sass doesn\'t otherwise support.\
            \n$result: unquote(\"b; c {d: e\");\
            \na {\
            \n  result: $result;\
            \n  length: str-length($result);\
            \n  same: $result == \"b; c {d: e\";\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  result: b; c {d: e;\
        \n  length: 10;\
        \n  same: true;\
        \n}\
        \n"
    );
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: unquote($string: c)}\
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
    fn quoted() {
        assert_eq!(
            rsass(
                "a {b: unquote(\"c\")}\
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
    fn unquoted() {
        assert_eq!(
            rsass(
                "a {b: unquote(c)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c;\
        \n}\
        \n"
        );
    }
}
