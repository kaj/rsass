//! Tests auto-converted from "sass-spec/spec/core_functions/string/insert.hrx"

#[test]
fn combining_character() {
    assert_eq!(
        crate::rsass(
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
        crate::rsass(
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
    #[test]
    fn empty_source() {
        assert_eq!(
            crate::rsass(
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
            crate::rsass(
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
            crate::rsass(
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
            crate::rsass(
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
            crate::rsass(
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
        crate::rsass(
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

    // Ignoring "decimal", error tests are not supported yet.

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.
    mod test_type {

        // Ignoring "index", error tests are not supported yet.

        // Ignoring "insert", error tests are not supported yet.

        // Ignoring "string", error tests are not supported yet.
    }
}
mod index {
    mod negative {
        #[test]
        fn t1() {
            assert_eq!(
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
        #[test]
        fn t0() {
            assert_eq!(
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
        crate::rsass(
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
