//! Tests auto-converted from "sass-spec/spec/values/numbers/modulo"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/values/numbers/modulo/floats.hrx"
mod floats {
    #[allow(unused)]
    use super::rsass;
    mod larger {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn negative_negative() {
            assert_eq!(
                rsass(
                    "a {\
            \n  b: -6.3 % -2.4;\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: -1.5;\
        \n}\
        \n"
            );
        }
        #[test]
        fn negative_positive() {
            assert_eq!(
                rsass(
                    "a {\
            \n  b: -6.3 % 2.4;\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 0.9;\
        \n}\
        \n"
            );
        }
        #[test]
        fn positive_negative() {
            assert_eq!(
                rsass(
                    "a {\
            \n  b: 6.3 % -2.4;\
            \n}"
                )
                .unwrap(),
                "a {\
        \n  b: -0.9;\
        \n}\
        \n"
            );
        }
        #[test]
        fn positive_positive() {
            assert_eq!(
                rsass(
                    "a {\
            \n  b: 6.3 % 2.4;\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 1.5;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn negative_negative() {
        assert_eq!(
            rsass(
                "a {\
            \n  b: -1.2 % -4.7;\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: -1.2;\
        \n}\
        \n"
        );
    }
    #[test]
    fn negative_positive() {
        assert_eq!(
            rsass(
                "a {\
            \n  b: -1.2 % 4.7;\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 3.5;\
        \n}\
        \n"
        );
    }
    #[test]
    fn positive_negative() {
        assert_eq!(
            rsass(
                "a {\
            \n  b: 1.2 % -4.7;\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: -3.5;\
        \n}\
        \n"
        );
    }
    #[test]
    fn positive_positive() {
        assert_eq!(
            rsass(
                "a {\
            \n  b: 1.2 % 4.7;\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1.2;\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/values/numbers/modulo/ints.hrx"
mod ints {
    #[allow(unused)]
    use super::rsass;
    mod larger {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn negative_negative() {
            assert_eq!(
                rsass(
                    "a {\
            \n  b: -7 % -5;\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: -2;\
        \n}\
        \n"
            );
        }
        #[test]
        fn negative_positive() {
            assert_eq!(
                rsass(
                    "a {\
            \n  b: -7 % 5;\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 3;\
        \n}\
        \n"
            );
        }
        #[test]
        fn positive_negative() {
            assert_eq!(
                rsass(
                    "a {\
            \n  b: 6 % -5;\
            \n}"
                )
                .unwrap(),
                "a {\
        \n  b: -4;\
        \n}\
        \n"
            );
        }
        #[test]
        fn positive_positive() {
            assert_eq!(
                rsass(
                    "a {\
            \n  b: 6 % 5;\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 1;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn negative_negative() {
        assert_eq!(
            rsass(
                "a {\
            \n  b: -1 % -4;\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: -1;\
        \n}\
        \n"
        );
    }
    #[test]
    fn negative_positive() {
        assert_eq!(
            rsass(
                "a {\
            \n  b: -1 % 4;\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 3;\
        \n}\
        \n"
        );
    }
    #[test]
    fn positive_negative() {
        assert_eq!(
            rsass(
                "a {\
            \n  b: 1 % -4;\
            \n}"
            )
            .unwrap(),
            "a {\
        \n  b: -3;\
        \n}\
        \n"
        );
    }
    #[test]
    fn positive_positive() {
        assert_eq!(
            rsass(
                "a {\
            \n  b: 1 % 4;\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1;\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/values/numbers/modulo/zeros.hrx"
mod zeros {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn negative_negative() {
        assert_eq!(
            rsass(
                "a {\
            \n  b: -0 % -1;\
            \n}\
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
    fn negative_positive() {
        assert_eq!(
            rsass(
                "a {\
            \n  b: -0 % 1;\
            \n}\
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
    fn positive_negative() {
        assert_eq!(
            rsass(
                "a {\
            \n  b: +0 % -1;\
            \n}"
            )
            .unwrap(),
            "a {\
        \n  b: 0;\
        \n}\
        \n"
        );
    }
    #[test]
    fn positive_positive() {
        assert_eq!(
            rsass(
                "a {\
            \n  b: +0 % +1;\
            \n}\
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
    fn zero_divider() {
        assert_eq!(
            rsass(
                "a {\
            \n  b: inspect(1 % 0);\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: NaN;\
        \n}\
        \n"
        );
    }
}
