//! Tests auto-converted from "sass-spec/spec/core_functions/selector/append.hrx"

mod classes {
    #[test]
    fn double() {
        assert_eq!(
            crate::rsass(
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
            crate::rsass(
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

    // Ignoring "invalid", error tests are not supported yet.

    // Ignoring "leading_combinator", error tests are not supported yet.

    // Ignoring "namespace", error tests are not supported yet.

    // Ignoring "parent", error tests are not supported yet.

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.

    // Ignoring "universal", error tests are not supported yet.
}
mod format {
    mod input {
        #[test]
        fn initial() {
            assert_eq!(
                crate::rsass(
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
                crate::rsass(
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
    fn output() {
        assert_eq!(
            crate::rsass(
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
        crate::rsass(
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
        crate::rsass(
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
    #[test]
    fn descendant() {
        assert_eq!(
            crate::rsass(
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
            crate::rsass(
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
            crate::rsass(
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
