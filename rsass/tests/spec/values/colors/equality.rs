//! Tests auto-converted from "sass-spec/spec/values/colors/equality.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("equality")
}

mod test_false {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn different_space() {
        assert_eq!(
            runner()
                .ok("a {b: color(srgb 0 0 0) == color(srgb-linear 0 0 0)}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    fn different_type() {
        assert_eq!(
            runner().ok("a {b: red == unquote(\"red\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    mod legacy {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn different_space() {
            assert_eq!(
                runner().ok("a {b: red == hsl(0, 0%, 50%)}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        mod same_space {
            #[allow(unused)]
            use super::runner;

            mod hsl {
                #[allow(unused)]
                use super::runner;

                #[test]
                fn no_none() {
                    assert_eq!(
        runner().ok(
            "a {b: hsl(50deg 50% 80%) == hsl(51deg 50% 80%)}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
                }
                #[test]
                #[ignore] // unexepected error
                fn one_none() {
                    assert_eq!(
                        runner()
                            .ok("a {b: hsl(0 0% 80%) == hsl(none 0% 80%)}\n"),
                        "a {\
         \n  b: false;\
         \n}\n"
                    );
                }
            }
            mod hwb {
                #[allow(unused)]
                use super::runner;

                #[test]
                fn no_none() {
                    assert_eq!(
        runner().ok(
            "a {b: hwb(50deg 20% 30%) == hwb(51deg 20% 30%)}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
                }
                #[test]
                fn one_none() {
                    assert_eq!(
                        runner()
                            .ok("a {b: hwb(0 0% 0%) == hwb(none 0% 0%)}\n"),
                        "a {\
         \n  b: false;\
         \n}\n"
                    );
                }
            }
            mod rgb {
                #[allow(unused)]
                use super::runner;

                #[test]
                fn no_none() {
                    assert_eq!(
                        runner().ok("a {b: red == blue}\n"),
                        "a {\
         \n  b: false;\
         \n}\n"
                    );
                }
                #[test]
                #[ignore] // unexepected error
                fn one_none() {
                    assert_eq!(
                        runner().ok(
                            "a {b: rgb(0 100 200) == rgb(none 100 200)}\n"
                        ),
                        "a {\
         \n  b: false;\
         \n}\n"
                    );
                }
            }
        }
    }
    mod same_space {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn different_alpha() {
            assert_eq!(
                runner().ok(
                    "a {b: lab(50% 100 -100) == lab(50% 100 -100 / 0.9)}\n"
                ),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        fn no_none() {
            assert_eq!(
        runner().ok(
            "a {b: color(prophoto-rgb 0.1 0.2 0.3) == color(prophoto-rgb 0.1 0.2 0.4)}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
        }
        #[test]
        fn one_none() {
            assert_eq!(
                runner().ok("a {b: oklch(50% 0% none) == oklch(50% 0% 0)}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
    }
}
mod test_true {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn alpha() {
        assert_eq!(
        runner().ok(
            "a {b: color(prophoto-rgb 0.3 0.4 0.5 / 0.6) == color(prophoto-rgb 0.3 0.4 0.5 / 0.6)}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
    }
    mod legacy {
        #[allow(unused)]
        use super::runner;

        mod different_space {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn no_none() {
                assert_eq!(
                    runner().ok(
                        "a {b: purple == hsl(300, 100%, 25.098039215686%)}\n"
                    ),
                    "a {\
         \n  b: true;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn none() {
                assert_eq!(
                    runner()
                        .ok("a {b: gray == hsl(none 0% 50.196078431373%)}\n"),
                    "a {\
         \n  b: true;\
         \n}\n"
                );
            }
        }
        mod same_space {
            #[allow(unused)]
            use super::runner;

            mod hsl {
                #[allow(unused)]
                use super::runner;

                #[test]
                fn no_none() {
                    assert_eq!(
        runner().ok(
            "a {b: hsl(250, 80%, 20%) == hsl(250deg, 80%, 20%)}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
                }
                #[test]
                #[ignore] // unexepected error
                fn none() {
                    assert_eq!(
                        runner().ok(
                            "a {b: hsl(none 80% 20%) == hsl(none 80% 20%)}\n"
                        ),
                        "a {\
         \n  b: true;\
         \n}\n"
                    );
                }
                #[test]
                fn wrapped_hue() {
                    assert_eq!(
        runner().ok(
            "a {b: hsl(180, 80%, 20%) == hsl(540, 80%, 20%)}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
                }
            }
            mod hwb {
                #[allow(unused)]
                use super::runner;

                #[test]
                #[ignore] // wrong result
                fn no_none() {
                    assert_eq!(
        runner().ok(
            "a {b: hwb(250 30% 20%) == hwb(250deg 30% 20%)}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
                }
                #[test]
                fn none() {
                    assert_eq!(
                        runner().ok(
                            "a {b: hwb(none 30% 20%) == hwb(none 30% 20%)}\n"
                        ),
                        "a {\
         \n  b: true;\
         \n}\n"
                    );
                }
            }
            mod rgb {
                #[allow(unused)]
                use super::runner;

                #[test]
                fn no_none() {
                    assert_eq!(
                        runner().ok("a {b: #abcdef == #abcdef}\n"),
                        "a {\
         \n  b: true;\
         \n}\n"
                    );
                }
                #[test]
                #[ignore] // unexepected error
                fn none() {
                    assert_eq!(
                        runner().ok(
                            "a {b: rgb(50 none 120) == rgb(50 none 120)}\n"
                        ),
                        "a {\
         \n  b: true;\
         \n}\n"
                    );
                }
            }
        }
    }
    #[test]
    fn no_alpha_or_none() {
        assert_eq!(
            runner().ok("a {b: lab(66% 50 -100) == lab(66% 50 -100)}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    fn none() {
        assert_eq!(
            runner().ok("a {b: lch(100% 0% none) == lch(100% 0% none)}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn within_epsilon() {
        assert_eq!(
        runner().ok(
            "a {\
             \n  b: oklab(\
             \n    50.0000000000001 29.9999999999999 -20.0000000000001 / 0.999999999999\
             \n  ) == oklab(50 30 -20);\
             \n}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
    }
}
