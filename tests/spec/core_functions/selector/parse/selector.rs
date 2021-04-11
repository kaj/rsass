//! Tests auto-converted from "sass-spec/spec/core_functions/selector/parse/selector.hrx"

mod complex {
    #[test]
    fn adjacent_sibling() {
        assert_eq!(
            crate::rsass(
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
    fn child() {
        assert_eq!(
            crate::rsass(
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
    fn descendant() {
        assert_eq!(
            crate::rsass(
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
    fn sibling() {
        assert_eq!(
            crate::rsass(
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
fn compound() {
    assert_eq!(
        crate::rsass(
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
        crate::rsass(
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
    #[test]
    fn attribute() {
        assert_eq!(
            crate::rsass(
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
            crate::rsass(
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
            crate::rsass(
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
            crate::rsass(
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
        mod class {
            #[test]
            #[ignore] // unexepected error
            fn arg() {
                assert_eq!(
                    crate::rsass(
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
            fn combined_arg() {
                assert_eq!(
        crate::rsass(
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
                    crate::rsass(
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
            fn selector_arg() {
                assert_eq!(
                    crate::rsass(
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
            #[test]
            #[ignore] // unexepected error
            fn arg() {
                assert_eq!(
                    crate::rsass(
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
                    crate::rsass(
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
            fn selector_arg() {
                assert_eq!(
                    crate::rsass(
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
            crate::rsass(
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
            crate::rsass(
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
