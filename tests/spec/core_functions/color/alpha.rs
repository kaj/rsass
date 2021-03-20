//! Tests auto-converted from "sass-spec/spec/core_functions/color/alpha.hrx"

mod color {
    #[test]
    fn max() {
        assert_eq!(
            crate::rsass(
                "a {b: alpha(red)}\
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
    fn middle() {
        assert_eq!(
            crate::rsass(
                "a {b: alpha(rgba(red, 0.42))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.42;\
        \n}\
        \n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            crate::rsass(
                "a {b: alpha(rgba(red, 0))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0;\
        \n}\
        \n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            crate::rsass(
                "a {b: alpha($color: rgba(red, 0.73))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.73;\
        \n}\
        \n"
        );
    }
}
mod error {

    // Ignoring "quoted_string", error tests are not supported yet.

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.
    mod unquoted_string {

        // Ignoring "no_equals", error tests are not supported yet.

        // Ignoring "non_identifier_before_equals", error tests are not supported yet.
    }
}
mod filter {
    #[test]
    fn multi_args() {
        assert_eq!(
            crate::rsass(
                "a {b: alpha(c=d, e=f, g=h)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: alpha(c=d, e=f, g=h);\
        \n}\
        \n"
        );
    }
    #[test]
    fn one_arg() {
        assert_eq!(
            crate::rsass(
                "a {b: alpha(c=d)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: alpha(c=d);\
        \n}\
        \n"
        );
    }
    #[test]
    fn space_before_equals() {
        assert_eq!(
            crate::rsass(
                "a {b: alpha(unquote(\"c = d\"))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: alpha(c = d);\
        \n}\
        \n"
        );
    }
}
mod opacity {
    #[test]
    fn filter() {
        assert_eq!(
            crate::rsass(
                "a {b: opacity(10%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: opacity(10%);\
        \n}\
        \n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            crate::rsass(
                "a {b: opacity($color: rgba(red, 0.2))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.2;\
        \n}\
        \n"
        );
    }
    #[test]
    fn positional() {
        assert_eq!(
            crate::rsass(
                "a {b: opacity(rgba(red, 0.2))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.2;\
        \n}\
        \n"
        );
    }
}
