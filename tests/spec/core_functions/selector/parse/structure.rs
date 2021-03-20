//! Tests auto-converted from "sass-spec/spec/core_functions/selector/parse/structure.hrx"

mod decomposed {
    mod complex {
        #[test]
        fn mixed() {
            assert_eq!(
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
        #[test]
        fn mixed() {
            assert_eq!(
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
        #[test]
        fn mixed() {
            assert_eq!(
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
        #[test]
        fn mixed() {
            assert_eq!(
                crate::rsass(
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
                crate::rsass(
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
        crate::rsass(
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
    #[test]
    fn quoted() {
        assert_eq!(
            crate::rsass(
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
            crate::rsass(
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
