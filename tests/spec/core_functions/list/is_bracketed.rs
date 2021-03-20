//! Tests auto-converted from "sass-spec/spec/core_functions/list/is_bracketed.hrx"

mod bracketed {
    #[test]
    fn empty() {
        assert_eq!(
            crate::rsass(
                "a {b: is-bracketed([])}\
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
    fn multi() {
        assert_eq!(
            crate::rsass(
                "a {b: is-bracketed([1, 2, 3])}\
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
    fn single() {
        assert_eq!(
            crate::rsass(
                "a {b: is-bracketed([1])}\
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
mod error {

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.
}
mod unbracketed {
    #[test]
    fn empty() {
        assert_eq!(
            crate::rsass(
                "a {b: is-bracketed(())}\
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
    fn map() {
        assert_eq!(
            crate::rsass(
                "a {b: is-bracketed((c: d, e: f, g: h))}\
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
    fn multi() {
        assert_eq!(
            crate::rsass(
                "a {b: is-bracketed(1 2 3)}\
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
    fn non_list() {
        assert_eq!(
            crate::rsass(
                "a {b: is-bracketed(1)}\
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
    fn single() {
        assert_eq!(
            crate::rsass(
                "a {b: is-bracketed((1,))}\
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
