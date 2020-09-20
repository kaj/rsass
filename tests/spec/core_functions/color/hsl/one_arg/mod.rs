//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/one_arg"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/core_functions/color/hsl/one_arg/alpha.hrx"
mod alpha {
    #[allow(unused)]
    use super::rsass;
    mod clamped {
        #[allow(unused)]
        use super::rsass;
        mod alpha {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn above() {
                assert_eq!(
                    rsass(
                        "a {b: hsl(0 100% 50% / 1.1)}\
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
            #[ignore] // wrong result
            fn below() {
                assert_eq!(
                    rsass(
                        "a {b: hsl(0 100% 50% / -0.1)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: rgba(255, 0, 0, 0);\
        \n}\
        \n"
                );
            }
        }
        #[test]
        #[ignore] // wrong result
        fn lightness() {
            assert_eq!(
                rsass(
                    "a {b: hsl(0 100% 9999% / 0.5)}\
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
        #[ignore] // wrong result
        fn saturation() {
            assert_eq!(
                rsass(
                    "a {b: hsl(0 -0.1% 50% / 0.5)}\
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
    mod in_gamut {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn named() {
            assert_eq!(
                rsass(
                    "a {b: hsl($channels: 180 60% 50% / 0.4)}\
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
        #[ignore] // wrong result
        fn opaque() {
            assert_eq!(
                rsass(
                    "a {b: hsl(180 60% 50% / 1)}\
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
        #[ignore] // wrong result
        fn parenthesized() {
            assert_eq!(
        rsass(
            "// Extra parens shouldn\'t cause the slash to be forced into division.\
            \na {b: (hsl(180 60% 50% / 0.4))}\
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
        #[ignore] // wrong result
        fn partial() {
            assert_eq!(
                rsass(
                    "a {b: hsl(180 60% 50% / 0.5)}\
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
        #[ignore] // wrong result
        fn transparent() {
            assert_eq!(
                rsass(
                    "a {b: hsl(180 60% 50% / 0)}\
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
}

// From "sass-spec/spec/core_functions/color/hsl/one_arg/no_alpha.hrx"
mod no_alpha {
    #[allow(unused)]
    use super::rsass;
    mod clamped {
        #[allow(unused)]
        use super::rsass;
        mod lightness {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn above() {
                assert_eq!(
                    rsass(
                        "a {b: hsl(0 100% 500%)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: white;\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn below() {
                assert_eq!(
                    rsass(
                        "a {b: hsl(0 100% -100%)}\
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
        mod saturation {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn above() {
                assert_eq!(
                    rsass(
                        "a {b: hsl(0 500% 50%)}\
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
            #[ignore] // wrong result
            fn below() {
                assert_eq!(
                    rsass(
                        "a {b: hsl(0 -100% 50%)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: gray;\
        \n}\
        \n"
                );
            }
        }
    }
    mod in_gamut {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn blue() {
            assert_eq!(
                rsass(
                    "a {b: hsl(240 100% 50%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: blue;\
        \n}\
        \n"
            );
        }
        mod grayish {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn yellow() {
                assert_eq!(
                    rsass(
                        "a {b: hsl(60 60% 50%)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: #cccc33;\
        \n}\
        \n"
                );
            }
        }
        #[test]
        #[ignore] // wrong result
        fn red() {
            assert_eq!(
                rsass(
                    "a {b: hsl(0 100% 50%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: red;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn named() {
        assert_eq!(
            rsass(
                "a {b: hsl($channels: 0 100% 50%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: red;\
        \n}\
        \n"
        );
    }
    mod units {
        #[allow(unused)]
        use super::rsass;
        mod hue {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn deg() {
                assert_eq!(
                    rsass(
                        "a {b: hsl(0deg 100% 50%)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: red;\
        \n}\
        \n"
                );
            }
        }
        mod lightness {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn unitless() {
                assert_eq!(
                    rsass(
                        "a {b: hsl(0 100% 50)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: red;\
        \n}\
        \n"
                );
            }
        }
        mod saturation {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn unitless() {
                assert_eq!(
                    rsass(
                        "a {b: hsl(0 50 50%)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: #bf4040;\
        \n}\
        \n"
                );
            }
        }
    }
}

// From "sass-spec/spec/core_functions/color/hsl/one_arg/special_functions.hrx"
mod special_functions {
    #[allow(unused)]
    use super::rsass;
    mod alpha {
        #[allow(unused)]
        use super::rsass;
        mod calc {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn arg_1() {
                assert_eq!(
                    rsass(
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
            #[ignore] // wrong result
            fn arg_2() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
                    rsass(
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
        mod env {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn arg_1() {
                assert_eq!(
                    rsass(
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
            #[ignore] // wrong result
            fn arg_2() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
                    rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn arg_1() {
                assert_eq!(
                    rsass(
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
            #[ignore] // wrong result
            fn arg_2() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
                    rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn arg_1() {
                assert_eq!(
                    rsass(
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
            #[ignore] // wrong result
            fn arg_2() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
                    rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn t1_of_1() {
                assert_eq!(
                    rsass(
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
        rsass(
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
                    rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn arg_1() {
                assert_eq!(
                    rsass(
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
            #[ignore] // wrong result
            fn arg_2() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
                    rsass(
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
        #[allow(unused)]
        use super::rsass;
        mod calc {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn arg_1() {
                assert_eq!(
                    rsass(
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
            #[ignore] // wrong result
            fn arg_2() {
                assert_eq!(
                    rsass(
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
            #[ignore] // wrong result
            fn arg_3() {
                assert_eq!(
                    rsass(
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
        mod env {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn arg_1() {
                assert_eq!(
                    rsass(
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
            #[ignore] // wrong result
            fn arg_2() {
                assert_eq!(
                    rsass(
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
            #[ignore] // wrong result
            fn arg_3() {
                assert_eq!(
                    rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn arg_1() {
                assert_eq!(
                    rsass(
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
            #[ignore] // wrong result
            fn arg_2() {
                assert_eq!(
                    rsass(
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
            #[ignore] // wrong result
            fn arg_3() {
                assert_eq!(
                    rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn arg_1() {
                assert_eq!(
                    rsass(
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
            #[ignore] // wrong result
            fn arg_2() {
                assert_eq!(
                    rsass(
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
            #[ignore] // wrong result
            fn arg_3() {
                assert_eq!(
                    rsass(
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
            #[allow(unused)]
            use super::rsass;
            mod argument {
                #[allow(unused)]
                use super::rsass;
                mod var {
                    #[allow(unused)]
                    use super::rsass;
                    mod arg_1 {
                        #[allow(unused)]
                        use super::rsass;
                        mod of {
                            #[allow(unused)]
                            use super::rsass;
                            #[test]
                            fn arg_1() {
                                assert_eq!(
                                    rsass(
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
        rsass(
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
                        #[allow(unused)]
                        use super::rsass;
                        mod of {
                            #[allow(unused)]
                            use super::rsass;
                            #[test]
                            fn arg_2() {
                                assert_eq!(
                                    rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn arg_1() {
                assert_eq!(
                    rsass(
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
            #[ignore] // wrong result
            fn arg_2() {
                assert_eq!(
                    rsass(
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
            #[ignore] // wrong result
            fn arg_3() {
                assert_eq!(
                    rsass(
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
}
