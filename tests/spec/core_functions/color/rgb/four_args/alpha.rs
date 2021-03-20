//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/four_args/alpha.hrx"

mod percent {
    mod above {
        #[test]
        fn max() {
            assert_eq!(
                crate::rsass(
                    "a {b: rgb(0, 0, 0, 250%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: black;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn max() {
        assert_eq!(
            crate::rsass(
                "a {b: rgb(0, 0, 0, 100%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: black;\
        \n}\
        \n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            crate::rsass(
                "a {b: rgb(0, 0, 0, 0%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(0, 0, 0, 0);\
        \n}\
        \n"
        );
    }
    #[test]
    fn negative() {
        assert_eq!(
            crate::rsass(
                "a {b: rgb(0, 0, 0, -10%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(0, 0, 0, 0);\
        \n}\
        \n"
        );
    }
    #[test]
    fn positive() {
        assert_eq!(
            crate::rsass(
                "a {b: rgb(0, 0, 0, 45.6%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(0, 0, 0, 0.456);\
        \n}\
        \n"
        );
    }
}
mod unitless {
    mod above {
        #[test]
        fn max() {
            assert_eq!(
                crate::rsass(
                    "a {b: rgb(0, 0, 0, 250)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: black;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn max() {
        assert_eq!(
            crate::rsass(
                "a {b: rgb(0, 0, 0, 1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: black;\
        \n}\
        \n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            crate::rsass(
                "a {b: rgb(0, 0, 0, 0)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(0, 0, 0, 0);\
        \n}\
        \n"
        );
    }
    #[test]
    fn negative() {
        assert_eq!(
            crate::rsass(
                "a {b: rgb(0, 0, 0, -10)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(0, 0, 0, 0);\
        \n}\
        \n"
        );
    }
    #[test]
    fn positive() {
        assert_eq!(
            crate::rsass(
                "a {b: rgb(0, 0, 0, 0.456)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(0, 0, 0, 0.456);\
        \n}\
        \n"
        );
    }
}
