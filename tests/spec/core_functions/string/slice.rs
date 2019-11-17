//! Tests auto-converted from "sass-spec/spec/core_functions/string/slice.hrx"

#[test]
fn combining_character() {
    assert_eq!(
        crate::rsass(
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
        crate::rsass(
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
    mod end {
        #[test]
        fn t0() {
            assert_eq!(
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
        #[test]
        fn t0() {
            assert_eq!(
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
    mod negative {
        #[test]
        fn t1() {
            assert_eq!(
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
        #[test]
        fn t0() {
            assert_eq!(
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
    mod decimal {
        #[test]
        #[ignore] // wrong error
        fn end() {
            assert_eq!(
                crate::rsass(
                    "a {b: str-slice(\"\", 1, 1.5)}\
             \n"
                )
                .unwrap_err(),
                "Error: 1.5 is not an int.\
         \n  ,\
         \n1 | a {b: str-slice(\"\", 1, 1.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn start() {
            assert_eq!(
                crate::rsass(
                    "a {b: str-slice(\"\", 0.5)}\
             \n"
                )
                .unwrap_err(),
                "Error: 0.5 is not an int.\
         \n  ,\
         \n1 | a {b: str-slice(\"\", 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
            );
        }
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: str-slice(\"cde\")}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $start-at.\
         \n  ,--> input.scss\
         \n1 | a {b: str-slice(\"cde\")}\
         \n  |       ^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function slice($string, $start-at, $end-at: -1) {\
         \n  |           ====================================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "a {b: str-slice(\"cde\", 1, 2, 3)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 3 arguments allowed, but 4 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: str-slice(\"cde\", 1, 2, 3)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function slice($string, $start-at, $end-at: -1) {\
         \n  |           ====================================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    mod test_type {
        #[test]
        fn end_at() {
            assert_eq!(
                crate::rsass(
                    "a {b: str-slice(\"cde\", 1, \"f\")}\
             \n"
                )
                .unwrap_err(),
                "Error: $end-at: \"f\" is not a number.\
         \n  ,\
         \n1 | a {b: str-slice(\"cde\", 1, \"f\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
            );
        }
        #[test]
        fn start_at() {
            assert_eq!(
                crate::rsass(
                    "a {b: str-slice(\"cde\", \"f\")}\
             \n"
                )
                .unwrap_err(),
                "Error: $start-at: \"f\" is not a number.\
         \n  ,\
         \n1 | a {b: str-slice(\"cde\", \"f\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
            );
        }
        #[test]
        fn string() {
            assert_eq!(
                crate::rsass(
                    "a {b: str-slice(1, 2)}\
             \n"
                )
                .unwrap_err(),
                "Error: $string: 1 is not a string.\
         \n  ,\
         \n1 | a {b: str-slice(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
            );
        }
    }
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
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
    mod negative {
        #[test]
        fn t1() {
            assert_eq!(
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
        #[test]
        fn t0() {
            assert_eq!(
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
        crate::rsass(
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
