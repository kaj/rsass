//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgba/four_args"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/core_functions/color/rgba/four_args/clamped.hrx"
mod clamped {
    #[allow(unused)]
    use super::rsass;
    mod alpha {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn above() {
            assert_eq!(
                rsass(
                    "a {b: rgba(0, 0, 0, 1.1)}\
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
        fn below() {
            assert_eq!(
                rsass(
                    "a {b: rgba(0, 0, 0, -0.1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(0, 0, 0, 0);\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn blue() {
        assert_eq!(
            rsass(
                "a {b: rgba(0, 0, 9999, 0.5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(0, 0, 255, 0.5);\
        \n}\
        \n"
        );
    }
    #[test]
    fn green() {
        assert_eq!(
            rsass(
                "a {b: rgba(0, -1, 0, 0.5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(0, 0, 0, 0.5);\
        \n}\
        \n"
        );
    }
    #[test]
    fn red() {
        assert_eq!(
            rsass(
                "a {b: rgba(256, 0, 0, 0.5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(255, 0, 0, 0.5);\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/color/rgba/four_args/in_gamut.hrx"
mod in_gamut {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: rgba($red: 0, $green: 255, $blue: 127, $alpha: 0.3)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(0, 255, 127, 0.3);\
        \n}\
        \n"
        );
    }
    #[test]
    fn opaque() {
        assert_eq!(
            rsass(
                "a {b: rgba(190, 173, 237, 1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #beaded;\
        \n}\
        \n"
        );
    }
    #[test]
    fn partial() {
        assert_eq!(
            rsass(
                "a {b: rgba(18, 52, 86, 0.5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(18, 52, 86, 0.5);\
        \n}\
        \n"
        );
    }
    #[test]
    fn transparent() {
        assert_eq!(
            rsass(
                "a {b: rgba(0, 255, 127, 0)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(0, 255, 127, 0);\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/color/rgba/four_args/special_functions.hrx"
mod special_functions {
    #[allow(unused)]
    use super::rsass;
    mod calc {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn arg_1() {
            assert_eq!(
                rsass(
                    "a {b: rgba(calc(1), 2, 3, 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(calc(1), 2, 3, 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                rsass(
                    "a {b: rgba(1, calc(2), 3, 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(1, calc(2), 3, 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                rsass(
                    "a {b: rgba(1, 2, calc(3), 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(1, 2, calc(3), 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                rsass(
                    "a {b: rgba(1, 2, 3, calc(0.4))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(1, 2, 3, calc(0.4));\
        \n}\
        \n"
            );
        }
    }
    mod env {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn arg_1() {
            assert_eq!(
                rsass(
                    "a {b: rgba(env(--foo), 2, 3, 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(env(--foo), 2, 3, 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                rsass(
                    "a {b: rgba(1, env(--foo), 3, 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(1, env(--foo), 3, 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                rsass(
                    "a {b: rgba(1, 2, env(--foo), 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(1, 2, env(--foo), 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                rsass(
                    "a {b: rgba(1, 2, 3, env(--foo))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(1, 2, 3, env(--foo));\
        \n}\
        \n"
            );
        }
    }
    mod max {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn arg_1() {
            assert_eq!(
                rsass(
                    "a {b: rgba(max(1), 2, 3, 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(max(1), 2, 3, 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                rsass(
                    "a {b: rgba(1, max(2), 3, 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(1, max(2), 3, 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                rsass(
                    "a {b: rgba(1, 2, max(3), 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(1, 2, max(3), 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                rsass(
                    "a {b: rgba(1, 2, 3, max(0.4))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(1, 2, 3, max(0.4));\
        \n}\
        \n"
            );
        }
    }
    mod min {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn arg_1() {
            assert_eq!(
                rsass(
                    "a {b: rgba(min(1), 2, 3, 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(min(1), 2, 3, 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                rsass(
                    "a {b: rgba(1, min(2), 3, 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(1, min(2), 3, 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                rsass(
                    "a {b: rgba(1, 2, min(3), 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(1, 2, min(3), 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                rsass(
                    "a {b: rgba(1, 2, 3, min(0.4))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(1, 2, 3, min(0.4));\
        \n}\
        \n"
            );
        }
    }
    mod var {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn arg_1() {
            assert_eq!(
                rsass(
                    "a {b: rgba(var(--foo), 2, 3, 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(var(--foo), 2, 3, 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                rsass(
                    "a {b: rgba(1, var(--foo), 3, 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(1, var(--foo), 3, 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                rsass(
                    "a {b: rgba(1, 2, var(--foo), 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(1, 2, var(--foo), 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                rsass(
                    "a {b: rgba(1, 2, 3, var(--foo))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(1, 2, 3, var(--foo));\
        \n}\
        \n"
            );
        }
    }
}
