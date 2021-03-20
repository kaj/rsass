//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/one_arg/special_functions.hrx"

mod alpha {
    mod calc {
        #[test]
        fn arg_1() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(calc(1) 2% 3% / 0.4)}\
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
                crate::rsass(
                    "a {b: hsl(1 calc(2%) 3% / 0.4)}\
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
                crate::rsass(
                    "a {b: hsl(1 2% calc(3%) / 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1 2% calc(3%)/0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(1 2% 3% / calc(0.4))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1 2% 3%/calc(0.4));\
        \n}\
        \n"
            );
        }
    }
    mod clamp {
        #[test]
        fn arg_1() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(clamp(1, 2, 3) 2% 3% / 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(clamp(1, 2, 3), 2%, 3%, 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(1 clamp(2%, 3%, 4%) 3% / 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1, clamp(2%, 3%, 4%), 3%, 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(1 2% clamp(3%, 4%, 5%) / 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1 2% clamp(3%, 4%, 5%)/0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(1 2% 3% / clamp(0.4, 0.5, 0.6))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1 2% 3%/clamp(0.4, 0.5, 0.6));\
        \n}\
        \n"
            );
        }
    }
    mod env {
        #[test]
        fn arg_1() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(env(--foo) 2% 3% / 0.4)}\
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
                crate::rsass(
                    "a {b: hsl(1 env(--foo) 3% / 0.4)}\
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
                crate::rsass(
                    "a {b: hsl(1 2% env(--foo) / 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1 2% env(--foo)/0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(1 2% 3% / env(--foo))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1 2% 3%/env(--foo));\
        \n}\
        \n"
            );
        }
    }
    mod max {
        #[test]
        fn arg_1() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(max(1) 2% 3% / 0.4)}\
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
                crate::rsass(
                    "a {b: hsl(1 max(2%) 3% / 0.4)}\
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
                crate::rsass(
                    "a {b: hsl(1 2% max(3%) / 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1 2% max(3%)/0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(1 2% 3% / max(0.4))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1 2% 3%/max(0.4));\
        \n}\
        \n"
            );
        }
    }
    mod min {
        #[test]
        fn arg_1() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(min(1) 2% 3% / 0.4)}\
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
                crate::rsass(
                    "a {b: hsl(1 min(2%) 3% / 0.4)}\
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
                crate::rsass(
                    "a {b: hsl(1 2% min(3%) / 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1 2% min(3%)/0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(1 2% 3% / min(0.4))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1 2% 3%/min(0.4));\
        \n}\
        \n"
            );
        }
    }
    mod multi_argument_var {
        #[test]
        fn t1_of_1() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(var(--foo) / 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(var(--foo)/0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn t1_of_2() {
            assert_eq!(
        crate::rsass(
            "// var() is substituted before parsing, so it may contain multiple arguments.\
            \na {b: hsl(var(--foo) 50% / 0.4)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: hsl(var(--foo) 50%/0.4);\
        \n}\
        \n"
    );
        }
        #[test]
        fn t2_of_2() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(0 var(--foo) / 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(0 var(--foo)/0.4);\
        \n}\
        \n"
            );
        }
    }
    mod var {
        #[test]
        fn arg_1() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(var(--foo) 2% 3% / 0.4)}\
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
                crate::rsass(
                    "a {b: hsl(1 var(--foo) 3% / 0.4)}\
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
                crate::rsass(
                    "a {b: hsl(1 2% var(--foo) / 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1 2% var(--foo)/0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(1 2% 3% / var(--foo))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1 2% 3%/var(--foo));\
        \n}\
        \n"
            );
        }
    }
}
mod no_alpha {
    mod calc {
        #[test]
        fn arg_1() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(calc(1) 2% 3%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(calc(1), 2%, 3%);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(1 calc(2%) 3%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1, calc(2%), 3%);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(1 2% calc(3%))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1, 2%, calc(3%));\
        \n}\
        \n"
            );
        }
    }
    mod clamp {
        #[test]
        fn arg_1() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(clamp(1, 2, 3) 2% 3%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(clamp(1, 2, 3), 2%, 3%);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(1 clamp(2%, 3%, 4%) 3%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1, clamp(2%, 3%, 4%), 3%);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(1 2% clamp(3%, 4%, 5%))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1, 2%, clamp(3%, 4%, 5%));\
        \n}\
        \n"
            );
        }
    }
    mod env {
        #[test]
        fn arg_1() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(env(--foo) 2% 3%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(env(--foo), 2%, 3%);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(1 env(--foo) 3%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1, env(--foo), 3%);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(1 2% env(--foo))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1, 2%, env(--foo));\
        \n}\
        \n"
            );
        }
    }
    mod max {
        #[test]
        fn arg_1() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(max(1) 2% 3%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(max(1), 2%, 3%);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(1 max(2%) 3%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1, max(2%), 3%);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(1 2% max(3%))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1, 2%, max(3%));\
        \n}\
        \n"
            );
        }
    }
    mod min {
        #[test]
        fn arg_1() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(min(1) 2% 3%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(min(1), 2%, 3%);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(1 min(2%) 3%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1, min(2%), 3%);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(1 2% min(3%))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1, 2%, min(3%));\
        \n}\
        \n"
            );
        }
    }
    mod multi {
        mod argument {
            mod var {
                mod arg_1 {
                    mod of {
                        #[test]
                        fn arg_1() {
                            assert_eq!(
                                crate::rsass(
                                    "a {b: hsl(var(--foo))}\
            \n"
                                )
                                .unwrap(),
                                "a {\
        \n  b: hsl(var(--foo));\
        \n}\
        \n"
                            );
                        }
                        #[test]
                        fn arg_2() {
                            assert_eq!(
        crate::rsass(
            "// var() is substituted before parsing, so it may contain multiple arguments.\
            \na {b: hsl(var(--foo) 50%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: hsl(var(--foo) 50%);\
        \n}\
        \n"
    );
                        }
                    }
                }
                mod arg_2 {
                    mod of {
                        #[test]
                        fn arg_2() {
                            assert_eq!(
                                crate::rsass(
                                    "a {b: hsl(0 var(--foo))}\
            \n"
                                )
                                .unwrap(),
                                "a {\
        \n  b: hsl(0 var(--foo));\
        \n}\
        \n"
                            );
                        }
                    }
                }
            }
        }
    }
    mod var {
        #[test]
        fn arg_1() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(var(--foo) 2% 3%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(var(--foo), 2%, 3%);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(1 var(--foo) 3%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1, var(--foo), 3%);\
        \n}\
        \n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsl(1 2% var(--foo))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: hsl(1, 2%, var(--foo));\
        \n}\
        \n"
            );
        }
    }
}
