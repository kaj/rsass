//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

mod error;

mod four_args;

// From "sass-spec/spec/core_functions/color/rgb/multi_argument_var.hrx"
mod multi_argument_var {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn t1_of_1() {
        assert_eq!(
            rsass(
                "a {b: rgb(var(--foo))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgb(var(--foo));\
        \n}\
        \n"
        );
    }
    #[test]
    fn t1_of_2() {
        assert_eq!(
            rsass(
                "a {b: rgb(var(--foo), 0.4)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgb(var(--foo), 0.4);\
        \n}\
        \n"
        );
    }
    #[test]
    fn t1_of_3() {
        assert_eq!(
        rsass(
            "// var() is substituted before parsing, so it may contain multiple arguments.\
            \na {b: rgb(var(--foo), 3, 0.4)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgb(var(--foo), 3, 0.4);\
        \n}\
        \n"
    );
    }
    #[test]
    fn t2_of_2() {
        assert_eq!(
            rsass(
                "a {b: rgb(1, var(--foo))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgb(1, var(--foo));\
        \n}\
        \n"
        );
    }
    #[test]
    fn t2_of_3() {
        assert_eq!(
            rsass(
                "a {b: rgb(1, var(--foo), 0.4)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgb(1, var(--foo), 0.4);\
        \n}\
        \n"
        );
    }
    #[test]
    fn t3_of_3() {
        assert_eq!(
            rsass(
                "a {b: rgb(1, 2, var(--foo))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgb(1, 2, var(--foo));\
        \n}\
        \n"
        );
    }
}

mod one_arg;

mod three_args;

// From "sass-spec/spec/core_functions/color/rgb/two_args.hrx"
mod two_args {
    #[allow(unused)]
    use super::rsass;
    mod clamped {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn opaque() {
            assert_eq!(
                rsass(
                    "a {b: rgb(#123, 1.1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #112233;\
        \n}\
        \n"
            );
        }
        #[test]
        fn transparent() {
            assert_eq!(
                rsass(
                    "a {b: rgb(#123, -0.1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(17, 34, 51, 0);\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: rgb($color: #123, $alpha: 0.5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(17, 34, 51, 0.5);\
        \n}\
        \n"
        );
    }
    mod opaque_to {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn opaque() {
            assert_eq!(
                rsass(
                    "a {b: rgb(#123, 1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #112233;\
        \n}\
        \n"
            );
        }
        #[test]
        fn partial() {
            assert_eq!(
                rsass(
                    "a {b: rgb(#123, 0.5)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(17, 34, 51, 0.5);\
        \n}\
        \n"
            );
        }
        #[test]
        fn transparent() {
            assert_eq!(
                rsass(
                    "a {b: rgb(#123, 0)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(17, 34, 51, 0);\
        \n}\
        \n"
            );
        }
    }
    mod partial_to {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn opaque() {
            assert_eq!(
                rsass(
                    "a {b: rgb(rgba(0, 0, 255, 0.3), 1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: blue;\
        \n}\
        \n"
            );
        }
        #[test]
        fn partial() {
            assert_eq!(
                rsass(
                    "a {b: rgb(rgba(0, 0, 255, 0.3), 0.5)}\
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
        fn transparent() {
            assert_eq!(
                rsass(
                    "a {b: rgb(rgba(0, 0, 255, 0.3), 0)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(0, 0, 255, 0);\
        \n}\
        \n"
            );
        }
    }
    mod transparent_to {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn opaque() {
            assert_eq!(
                rsass(
                    "a {b: rgb(transparent, 1)}\
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
        fn partial() {
            assert_eq!(
                rsass(
                    "a {b: rgb(transparent, 0.5)}\
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
        fn transparent() {
            assert_eq!(
                rsass(
                    "a {b: rgb(transparent, 0)}\
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
}
