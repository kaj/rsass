//! Tests auto-converted from "sass-spec/spec/core_functions/selector/replace.hrx"

#[test]
#[ignore] // wrong result
fn complex() {
    assert_eq!(
        crate::rsass(
            "a {b: selector-replace(\"c d\", \"d\", \"e f\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c e f, e c f;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn compound() {
    assert_eq!(
        crate::rsass(
            "a {b: selector-replace(\"c.d\", \"c\", \"e\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: e.d;\
        \n}\
        \n"
    );
}
mod error {
    mod extendee {
        mod complex {

            // Ignoring "list", error tests are not supported yet.

            // Ignoring "string", error tests are not supported yet.
        }

        // Ignoring "invalid", error tests are not supported yet.

        // Ignoring "parent", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }
    mod extender {

        // Ignoring "invalid", error tests are not supported yet.

        // Ignoring "parent", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }
    mod selector {

        // Ignoring "invalid", error tests are not supported yet.

        // Ignoring "parent", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.
}
mod format {
    mod input {
        mod multiple_extendees {
            #[test]
            #[ignore] // wrong result
            fn compound() {
                assert_eq!(
                    crate::rsass(
                        "a {b: selector-replace(\"c.d\", \"c.d\", \".e\")}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: .e;\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn list() {
                assert_eq!(
                    crate::rsass(
                        "a {b: selector-replace(\"c.d\", \"c, .d\", \".e\")}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: .e;\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn list_of_compound() {
                assert_eq!(
        crate::rsass(
            "a {b: selector-replace(\"c.d.e.f\", \"c.d, .e.f\", \".g\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: .g;\
        \n}\
        \n"
    );
            }
        }
        mod non_string {
            #[test]
            #[ignore] // wrong result
            fn extendee() {
                assert_eq!(
        crate::rsass(
            "a {b: selector-replace(\"c.d\", (c, \".d\"), \".e\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: .e;\
        \n}\
        \n"
    );
            }
            #[test]
            #[ignore] // wrong result
            fn extender() {
                assert_eq!(
                    crate::rsass(
                        "a {b: selector-replace(\"c\", \"c\", (d, e f))}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: d, e f;\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn selector() {
                assert_eq!(
                    crate::rsass(
                        "a {b: selector-replace((c, d c), \"c\", \"e\")}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: e, d e;\
        \n}\
        \n"
                );
            }
        }
    }
    #[test]
    #[ignore] // wrong result
    fn output() {
        assert_eq!(
            crate::rsass(
                "$result: selector-replace(\"c d, e f\", \"g\", \"g\");\
            \na {\
            \n  result: $result;\
            \n  structure: $result == (\"c\" \"d\", \"e\" \"f\");\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  result: c d, e f;\
        \n  structure: true;\
        \n}\
        \n"
        );
    }
}
#[test]
#[ignore] // wrong result
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: selector-replace($selector: \"c.d\", $original: \"c\", $replacement: \"e\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: e.d;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn no_op() {
    assert_eq!(
        crate::rsass(
            "a {b: selector-replace(\"c\", \"d\", \"e\")}\
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
#[ignore] // wrong result
fn partial_no_op() {
    assert_eq!(
        crate::rsass(
            "a {b: selector-replace(\"c, d\", \"d\", \"e\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c, e;\
        \n}\
        \n"
    );
}
mod selector_pseudo {
    #[test]
    #[ignore] // wrong result
    fn matches() {
        assert_eq!(
            crate::rsass(
                "a {b: selector-replace(\":matches(c)\", \"c\", \"d\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: :matches(d);\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn not() {
        assert_eq!(
            crate::rsass(
                "a {b: selector-replace(\":not(c)\", \"c\", \"d\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: :not(d);\
        \n}\
        \n"
        );
    }
}
#[test]
#[ignore] // wrong result
fn simple() {
    assert_eq!(
        crate::rsass(
            "a {b: selector-replace(\"c\", \"c\", \"d\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: d;\
        \n}\
        \n"
    );
}
