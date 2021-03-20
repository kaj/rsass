//! Tests auto-converted from "sass-spec/spec/css/custom_properties/syntax.hrx"

mod double_dash {
    #[test]
    fn declare() {
        assert_eq!(
            crate::rsass(
                "a {--: b}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  --: b;\
        \n}\
        \n"
        );
    }
    #[test]
    fn test_use() {
        assert_eq!(
            crate::rsass(
                "a {b: var(--)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: var(--);\
        \n}\
        \n"
        );
    }
}
mod initial_digit {
    #[test]
    fn declare() {
        assert_eq!(
            crate::rsass(
                "a {--1: b}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  --1: b;\
        \n}\
        \n"
        );
    }
    #[test]
    fn test_use() {
        assert_eq!(
            crate::rsass(
                "a {b: var(--1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: var(--1);\
        \n}\
        \n"
        );
    }
}
mod triple_dash {
    #[test]
    fn declare() {
        assert_eq!(
            crate::rsass(
                "a {---: b}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  ---: b;\
        \n}\
        \n"
        );
    }
    #[test]
    fn test_use() {
        assert_eq!(
            crate::rsass(
                "a {b: var(---)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: var(---);\
        \n}\
        \n"
        );
    }
}
