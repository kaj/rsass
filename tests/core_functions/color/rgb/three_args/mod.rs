//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/three_args"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/core_functions/color/rgb/three_args/percents.hrx"
mod percents {
    #[allow(unused)]
    use super::rsass;
    mod all {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn percent() {
            assert_eq!(
                rsass("a {b: rgb(7.1%, 20.4%, 33.9%)}\n").unwrap(),
                "a {\n  b: #123456;\n}\n"
            );
        }
    }
    #[test]
    fn boundaries() {
        assert_eq!(
            rsass("a {b: rgb(0%, 100%, 50%)}\n").unwrap(),
            "a {\n  b: #00ff80;\n}\n"
        );
    }
    mod clamped {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn blue() {
            assert_eq!(
                rsass("a {b: rgb(0, 0, 200%)}\n").unwrap(),
                "a {\n  b: blue;\n}\n"
            );
        }
        #[test]
        fn green() {
            assert_eq!(
                rsass("a {b: rgb(0, -0.1%, 0)}\n").unwrap(),
                "a {\n  b: black;\n}\n"
            );
        }
        #[test]
        fn red() {
            assert_eq!(
                rsass("a {b: rgb(100.1%, 0, 0)}\n").unwrap(),
                "a {\n  b: red;\n}\n"
            );
        }
    }
    mod percent {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn green() {
            assert_eq!(
                rsass("a {b: rgb(190, 68%, 237)}\n").unwrap(),
                "a {\n  b: #beaded;\n}\n"
            );
        }
    }
    mod unitless {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn green() {
            assert_eq!(
                rsass("a {b: rgb(74.7%, 173, 93%)}\n").unwrap(),
                "a {\n  b: #beaded;\n}\n"
            );
        }
    }
}

// From "sass-spec/spec/core_functions/color/rgb/three_args/special_functions.hrx"
mod special_functions {
    #[allow(unused)]
    use super::rsass;
    mod calc {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn arg_1() {
            assert_eq!(
                rsass("a {b: rgb(calc(1), 2, 3)}\n").unwrap(),
                "a {\n  b: rgb(calc(1), 2, 3);\n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                rsass("a {b: rgb(1, calc(2), 3)}\n").unwrap(),
                "a {\n  b: rgb(1, calc(2), 3);\n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                rsass("a {b: rgb(1, 2, calc(3))}\n").unwrap(),
                "a {\n  b: rgb(1, 2, calc(3));\n}\n"
            );
        }
    }
    mod env {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn arg_1() {
            assert_eq!(
                rsass("a {b: rgb(env(--foo), 2, 3)}\n").unwrap(),
                "a {\n  b: rgb(env(--foo), 2, 3);\n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                rsass("a {b: rgb(1, env(--foo), 3)}\n").unwrap(),
                "a {\n  b: rgb(1, env(--foo), 3);\n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                rsass("a {b: rgb(1, 2, env(--foo))}\n").unwrap(),
                "a {\n  b: rgb(1, 2, env(--foo));\n}\n"
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
                rsass("a {b: rgb(max(1), 2, 3)}\n").unwrap(),
                "a {\n  b: rgb(max(1), 2, 3);\n}\n"
            );
        }
        #[test]
        #[ignore] // failing
        fn arg_2() {
            assert_eq!(
                rsass("a {b: rgb(1, max(2), 3)}\n").unwrap(),
                "a {\n  b: rgb(1, max(2), 3);\n}\n"
            );
        }
        #[test]
        #[ignore] // failing
        fn arg_3() {
            assert_eq!(
                rsass("a {b: rgb(1, 2, max(3))}\n").unwrap(),
                "a {\n  b: rgb(1, 2, max(3));\n}\n"
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
                rsass("a {b: rgb(min(1), 2, 3)}\n").unwrap(),
                "a {\n  b: rgb(min(1), 2, 3);\n}\n"
            );
        }
        #[test]
        #[ignore] // failing
        fn arg_2() {
            assert_eq!(
                rsass("a {b: rgb(1, min(2), 3)}\n").unwrap(),
                "a {\n  b: rgb(1, min(2), 3);\n}\n"
            );
        }
        #[test]
        #[ignore] // failing
        fn arg_3() {
            assert_eq!(
                rsass("a {b: rgb(1, 2, min(3))}\n").unwrap(),
                "a {\n  b: rgb(1, 2, min(3));\n}\n"
            );
        }
    }
    mod var {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn arg_1() {
            assert_eq!(
                rsass("a {b: rgb(var(--foo), 2, 3)}\n").unwrap(),
                "a {\n  b: rgb(var(--foo), 2, 3);\n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                rsass("a {b: rgb(1, var(--foo), 3)}\n").unwrap(),
                "a {\n  b: rgb(1, var(--foo), 3);\n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                rsass("a {b: rgb(1, 2, var(--foo))}\n").unwrap(),
                "a {\n  b: rgb(1, 2, var(--foo));\n}\n"
            );
        }
    }
}

// From "sass-spec/spec/core_functions/color/rgb/three_args/unitless.hrx"
mod unitless {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn beaded() {
        assert_eq!(
            rsass("a {b: rgb(190, 173, 237)}\n").unwrap(),
            "a {\n  b: #beaded;\n}\n"
        );
    }
    mod clamped {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn blue() {
            assert_eq!(
                rsass("a {b: rgb(0, 0, 9999)}\n").unwrap(),
                "a {\n  b: blue;\n}\n"
            );
        }
        #[test]
        fn green() {
            assert_eq!(
                rsass("a {b: rgb(0, -1, 0)}\n").unwrap(),
                "a {\n  b: black;\n}\n"
            );
        }
        #[test]
        fn red() {
            assert_eq!(
                rsass("a {b: rgb(256, 0, 0)}\n").unwrap(),
                "a {\n  b: red;\n}\n"
            );
        }
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass("a {b: rgb($red: 0, $green: 255, $blue: 127)}\n").unwrap(),
            "a {\n  b: springgreen;\n}\n"
        );
    }
    #[test]
    fn numbers() {
        assert_eq!(
            rsass("a {b: rgb(18, 52, 86)}\n").unwrap(),
            "a {\n  b: #123456;\n}\n"
        );
    }
    #[test]
    fn springgreen() {
        assert_eq!(
            rsass("a {b: rgb(0, 255, 127)}\n").unwrap(),
            "a {\n  b: springgreen;\n}\n"
        );
    }
}
