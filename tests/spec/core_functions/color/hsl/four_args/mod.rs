//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/four_args"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/core_functions/color/hsl/four_args/alpha.hrx"
mod alpha {
    #[allow(unused)]
    use super::rsass;
    mod percent {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn above_max() {
            assert_eq!(
                rsass(
                    "a {b: hsl(0, 0, 0, 250%)}\
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
        fn max() {
            assert_eq!(
                rsass(
                    "a {b: hsl(0, 0, 0, 100%)}\
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
                rsass(
                    "a {b: hsl(0, 0, 0, 0%)}\
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
                rsass(
                    "a {b: hsl(0, 0, 0, -10%)}\
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
                rsass(
                    "a {b: hsl(0, 0, 0, 45.6%)}\
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
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn above_max() {
            assert_eq!(
                rsass(
                    "a {b: hsl(0, 0, 0, 250)}\
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
        fn max() {
            assert_eq!(
                rsass(
                    "a {b: hsl(0, 0, 0, 1)}\
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
                rsass(
                    "a {b: hsl(0, 0, 0, 0)}\
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
                rsass(
                    "a {b: hsl(0, 0, 0, -10)}\
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
                rsass(
                    "a {b: hsl(0, 0, 0, 0.456)}\
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
}

// From "sass-spec/spec/core_functions/color/hsl/four_args/clamped.hrx"
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
                    "a {b: hsl(0, 100%, 50%, 1.1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: red;\
        \n}\
        \n"
            );
        }
        #[test]
        fn below() {
            assert_eq!(
                rsass(
                    "a {b: rgba(0, 100%, 50%, -0.1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(0, 255, 128, 0);\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn blue() {
        assert_eq!(
            rsass(
                "a {b: hsl(0, 100%, 9999%, 0.5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(255, 255, 255, 0.5);\
        \n}\
        \n"
        );
    }
    #[test]
    fn saturation() {
        assert_eq!(
            rsass(
                "a {b: hsl(0, -0.1%, 50%, 0.5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(128, 128, 128, 0.5);\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/color/hsl/four_args/in_gamut.hrx"
mod in_gamut {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn named() {
        assert_eq!(
        rsass(
            "a {b: hsl($hue: 180, $saturation: 60%, $lightness: 50%, $alpha: 0.4)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(51, 204, 204, 0.4);\
        \n}\
        \n"
    );
    }
    #[test]
    fn opaque() {
        assert_eq!(
            rsass(
                "a {b: hsl(180, 60%, 50%, 1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #33cccc;\
        \n}\
        \n"
        );
    }
    #[test]
    fn partial() {
        assert_eq!(
            rsass(
                "a {b: hsl(180, 60%, 50%, 0.5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(51, 204, 204, 0.5);\
        \n}\
        \n"
        );
    }
    #[test]
    fn transparent() {
        assert_eq!(
            rsass(
                "a {b: hsl(180, 60%, 50%, 0)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(51, 204, 204, 0);\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/color/hsl/four_args/special_functions.hrx"
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
                    "a {b: hsl(calc(1), 2%, 3%, 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(calc(1), 2%, 3%, 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                rsass(
                    "a {b: hsl(1, calc(2%), 3%, 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1, calc(2%), 3%, 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                rsass(
                    "a {b: hsl(1, 2%, calc(3%), 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1, 2%, calc(3%), 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                rsass(
                    "a {b: hsl(1, 2%, 3%, calc(0.4))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1, 2%, 3%, calc(0.4));\
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
                    "a {b: hsl(env(--foo), 2%, 3%, 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(env(--foo), 2%, 3%, 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                rsass(
                    "a {b: hsl(1, env(--foo), 3%, 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1, env(--foo), 3%, 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                rsass(
                    "a {b: hsl(1, 2%, env(--foo), 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1, 2%, env(--foo), 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                rsass(
                    "a {b: hsl(1, 2%, 3%, env(--foo))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1, 2%, 3%, env(--foo));\
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
                    "a {b: hsl(max(1), 2%, 3%, 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(max(1), 2%, 3%, 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                rsass(
                    "a {b: hsl(1, max(2%), 3%, 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1, max(2%), 3%, 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                rsass(
                    "a {b: hsl(1, 2%, max(3%), 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1, 2%, max(3%), 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                rsass(
                    "a {b: hsl(1, 2%, 3%, max(0.4))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1, 2%, 3%, max(0.4));\
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
                    "a {b: hsl(min(1), 2%, 3%, 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(min(1), 2%, 3%, 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                rsass(
                    "a {b: hsl(1, min(2%), 3%, 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1, min(2%), 3%, 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                rsass(
                    "a {b: hsl(1, 2%, min(3%), 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1, 2%, min(3%), 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                rsass(
                    "a {b: hsl(1, 2%, 3%, min(0.4))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1, 2%, 3%, min(0.4));\
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
                    "a {b: hsl(var(--foo), 2%, 3%, 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(var(--foo), 2%, 3%, 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                rsass(
                    "a {b: hsl(1, var(--foo), 3%, 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1, var(--foo), 3%, 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                rsass(
                    "a {b: hsl(1, 2%, var(--foo), 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1, 2%, var(--foo), 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                rsass(
                    "a {b: hsl(1, 2%, 3%, var(--foo))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1, 2%, 3%, var(--foo));\
        \n}\
        \n"
            );
        }
    }
}
