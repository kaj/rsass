//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/four_args"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/core_functions/color/rgb/four_args/alpha.hrx"
mod alpha {
    #[allow(unused)]
    use super::rsass;
    mod percent {
        #[allow(unused)]
        use super::rsass;
        mod above {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn max() {
                assert_eq!(
                    rsass("a {b: rgb(0, 0, 0, 250%)}\n").unwrap(),
                    "a {\n  b: black;\n}\n"
                );
            }
        }
        #[test]
        fn max() {
            assert_eq!(
                rsass("a {b: rgb(0, 0, 0, 100%)}\n").unwrap(),
                "a {\n  b: black;\n}\n"
            );
        }
        #[test]
        fn min() {
            assert_eq!(
                rsass("a {b: rgb(0, 0, 0, 0%)}\n").unwrap(),
                "a {\n  b: rgba(0, 0, 0, 0);\n}\n"
            );
        }
        #[test]
        fn negative() {
            assert_eq!(
                rsass("a {b: rgb(0, 0, 0, -10%)}\n").unwrap(),
                "a {\n  b: rgba(0, 0, 0, 0);\n}\n"
            );
        }
        #[test]
        fn positive() {
            assert_eq!(
                rsass("a {b: rgb(0, 0, 0, 45.6%)}\n").unwrap(),
                "a {\n  b: rgba(0, 0, 0, 0.456);\n}\n"
            );
        }
    }
    mod unitless {
        #[allow(unused)]
        use super::rsass;
        mod above {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn max() {
                assert_eq!(
                    rsass("a {b: rgb(0, 0, 0, 250)}\n").unwrap(),
                    "a {\n  b: black;\n}\n"
                );
            }
        }
        #[test]
        fn max() {
            assert_eq!(
                rsass("a {b: rgb(0, 0, 0, 1)}\n").unwrap(),
                "a {\n  b: black;\n}\n"
            );
        }
        #[test]
        fn min() {
            assert_eq!(
                rsass("a {b: rgb(0, 0, 0, 0)}\n").unwrap(),
                "a {\n  b: rgba(0, 0, 0, 0);\n}\n"
            );
        }
        #[test]
        fn negative() {
            assert_eq!(
                rsass("a {b: rgb(0, 0, 0, -10)}\n").unwrap(),
                "a {\n  b: rgba(0, 0, 0, 0);\n}\n"
            );
        }
        #[test]
        fn positive() {
            assert_eq!(
                rsass("a {b: rgb(0, 0, 0, 0.456)}\n").unwrap(),
                "a {\n  b: rgba(0, 0, 0, 0.456);\n}\n"
            );
        }
    }
}

// From "sass-spec/spec/core_functions/color/rgb/four_args/clamped.hrx"
mod clamped {
    #[allow(unused)]
    use super::rsass;
    mod alpha {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn above() {
            assert_eq!(
                rsass("a {b: rgb(0, 0, 0, 1.1)}\n").unwrap(),
                "a {\n  b: black;\n}\n"
            );
        }
        #[test]
        fn below() {
            assert_eq!(
                rsass("a {b: rgb(0, 0, 0, -0.1)}\n").unwrap(),
                "a {\n  b: rgba(0, 0, 0, 0);\n}\n"
            );
        }
    }
    #[test]
    fn blue() {
        assert_eq!(
            rsass("a {b: rgb(0, 0, 9999, 0.5)}\n").unwrap(),
            "a {\n  b: rgba(0, 0, 255, 0.5);\n}\n"
        );
    }
    #[test]
    fn green() {
        assert_eq!(
            rsass("a {b: rgb(0, -1, 0, 0.5)}\n").unwrap(),
            "a {\n  b: rgba(0, 0, 0, 0.5);\n}\n"
        );
    }
    #[test]
    fn red() {
        assert_eq!(
            rsass("a {b: rgb(256, 0, 0, 0.5)}\n").unwrap(),
            "a {\n  b: rgba(255, 0, 0, 0.5);\n}\n"
        );
    }
}

// From "sass-spec/spec/core_functions/color/rgb/four_args/in_gamut.hrx"
mod in_gamut {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: rgb($red: 0, $green: 255, $blue: 127, $alpha: 0.3)}\n"
            )
            .unwrap(),
            "a {\n  b: rgba(0, 255, 127, 0.3);\n}\n"
        );
    }
    #[test]
    fn opaque() {
        assert_eq!(
            rsass("a {b: rgb(190, 173, 237, 1)}\n").unwrap(),
            "a {\n  b: #beaded;\n}\n"
        );
    }
    #[test]
    fn partial() {
        assert_eq!(
            rsass("a {b: rgb(18, 52, 86, 0.5)}\n").unwrap(),
            "a {\n  b: rgba(18, 52, 86, 0.5);\n}\n"
        );
    }
    #[test]
    fn transparent() {
        assert_eq!(
            rsass("a {b: rgb(0, 255, 127, 0)}\n").unwrap(),
            "a {\n  b: rgba(0, 255, 127, 0);\n}\n"
        );
    }
}

// From "sass-spec/spec/core_functions/color/rgb/four_args/special_functions.hrx"
mod special_functions {
    #[allow(unused)]
    use super::rsass;
    mod calc {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn arg_1() {
            assert_eq!(
                rsass("a {b: rgb(calc(1), 2, 3, 0.4)}\n").unwrap(),
                "a {\n  b: rgb(calc(1), 2, 3, 0.4);\n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                rsass("a {b: rgb(1, calc(2), 3, 0.4)}\n").unwrap(),
                "a {\n  b: rgb(1, calc(2), 3, 0.4);\n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                rsass("a {b: rgb(1, 2, calc(3), 0.4)}\n").unwrap(),
                "a {\n  b: rgb(1, 2, calc(3), 0.4);\n}\n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                rsass("a {b: rgb(1, 2, 3, calc(0.4))}\n").unwrap(),
                "a {\n  b: rgb(1, 2, 3, calc(0.4));\n}\n"
            );
        }
    }
    mod env {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn arg_1() {
            assert_eq!(
                rsass("a {b: rgb(env(--foo), 2, 3, 0.4)}\n").unwrap(),
                "a {\n  b: rgb(env(--foo), 2, 3, 0.4);\n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                rsass("a {b: rgb(1, env(--foo), 3, 0.4)}\n").unwrap(),
                "a {\n  b: rgb(1, env(--foo), 3, 0.4);\n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                rsass("a {b: rgb(1, 2, env(--foo), 0.4)}\n").unwrap(),
                "a {\n  b: rgb(1, 2, env(--foo), 0.4);\n}\n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                rsass("a {b: rgb(1, 2, 3, env(--foo))}\n").unwrap(),
                "a {\n  b: rgb(1, 2, 3, env(--foo));\n}\n"
            );
        }
    }
    mod max {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // failing
        fn arg_1() {
            assert_eq!(
                rsass("a {b: rgb(max(1), 2, 3, 0.4)}\n").unwrap(),
                "a {\n  b: rgb(max(1), 2, 3, 0.4);\n}\n"
            );
        }
        #[test]
        #[ignore] // failing
        fn arg_2() {
            assert_eq!(
                rsass("a {b: rgb(1, max(2), 3, 0.4)}\n").unwrap(),
                "a {\n  b: rgb(1, max(2), 3, 0.4);\n}\n"
            );
        }
        #[test]
        #[ignore] // failing
        fn arg_3() {
            assert_eq!(
                rsass("a {b: rgb(1, 2, max(3), 0.4)}\n").unwrap(),
                "a {\n  b: rgb(1, 2, max(3), 0.4);\n}\n"
            );
        }
        #[test]
        #[ignore] // failing
        fn arg_4() {
            assert_eq!(
                rsass("a {b: rgb(1, 2, 3, max(0.4))}\n").unwrap(),
                "a {\n  b: rgb(1, 2, 3, max(0.4));\n}\n"
            );
        }
    }
    mod min {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // failing
        fn arg_1() {
            assert_eq!(
                rsass("a {b: rgb(min(1), 2, 3, 0.4)}\n").unwrap(),
                "a {\n  b: rgb(min(1), 2, 3, 0.4);\n}\n"
            );
        }
        #[test]
        #[ignore] // failing
        fn arg_2() {
            assert_eq!(
                rsass("a {b: rgb(1, min(2), 3, 0.4)}\n").unwrap(),
                "a {\n  b: rgb(1, min(2), 3, 0.4);\n}\n"
            );
        }
        #[test]
        #[ignore] // failing
        fn arg_3() {
            assert_eq!(
                rsass("a {b: rgb(1, 2, min(3), 0.4)}\n").unwrap(),
                "a {\n  b: rgb(1, 2, min(3), 0.4);\n}\n"
            );
        }
        #[test]
        #[ignore] // failing
        fn arg_4() {
            assert_eq!(
                rsass("a {b: rgb(1, 2, 3, min(0.4))}\n").unwrap(),
                "a {\n  b: rgb(1, 2, 3, min(0.4));\n}\n"
            );
        }
    }
    mod var {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn arg_1() {
            assert_eq!(
                rsass("a {b: rgb(var(--foo), 2, 3, 0.4)}\n").unwrap(),
                "a {\n  b: rgb(var(--foo), 2, 3, 0.4);\n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                rsass("a {b: rgb(1, var(--foo), 3, 0.4)}\n").unwrap(),
                "a {\n  b: rgb(1, var(--foo), 3, 0.4);\n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                rsass("a {b: rgb(1, 2, var(--foo), 0.4)}\n").unwrap(),
                "a {\n  b: rgb(1, 2, var(--foo), 0.4);\n}\n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                rsass("a {b: rgb(1, 2, 3, var(--foo))}\n").unwrap(),
                "a {\n  b: rgb(1, 2, 3, var(--foo));\n}\n"
            );
        }
    }
}
