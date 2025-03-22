//! Tests auto-converted from "sass-spec/spec/core_functions/color/change/space.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("space")
}

#[test]
#[ignore] // unexepected error
fn identical() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.change(oklch(50% 0.2 0deg), $space: lab)}\n"),
        "a {\
         \n  b: oklch(50% 0.2 0deg);\
         \n}\n"
    );
}
mod legacy {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn to_legacy() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(pink, $saturation: 5%, $space: hsl)}\n"),
            "a {\
         \n  b: rgb(225.075, 221.925, 222.475);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn to_modern() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(pink, $chroma: 0.01, $space: oklch)}\n"),
            "a {\
         \n  b: rgb(217.7587741846, 208.8497862891, 210.1600712342);\
         \n}\n"
        );
    }
}
mod missing {
    use super::runner;

    mod arg {
        use super::runner;

        mod legacy {
            use super::runner;

            mod analogous {
                use super::runner;

                #[test]
                #[ignore] // unexepected error
                fn legacy() {
                    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(hsl(100deg 50% 50%), $hue: none, $space: hwb)}\n"
        ),
        "a {\
         \n  b: hsl(0, 50%, 50%);\
         \n}\n"
    );
                }
                #[test]
                #[ignore] // unexepected error
                fn modern() {
                    assert_eq!(
                        runner().ok("@use \"sass:color\";\
             \na {b: color.change(pink, $red: none, $space: display-p3)}\n"),
                        "a {\
         \n  b: rgb(0, 198.1453699836, 205.7002828396);\
         \n}\n"
                    );
                }
            }
            mod same {
                use super::runner;

                #[test]
                #[ignore] // unexepected error
                fn explicit() {
                    assert_eq!(
                        runner().ok("@use \"sass:color\";\
             \na {b: color.change(pink, $red: none, $space: rgb)}\n"),
                        "a {\
         \n  b: rgb(none 192 203);\
         \n}\n"
                    );
                }
                #[test]
                #[ignore] // unexepected error
                fn implicit() {
                    assert_eq!(
                        runner().ok("@use \"sass:color\";\
             \na {b: color.change(pink, $red: none)}\n"),
                        "a {\
         \n  b: rgb(none 192 203);\
         \n}\n"
                    );
                }
            }
        }
        mod modern {
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn analogous() {
                assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(color(srgb 0.1 0.2 0.3), $red: none, $space: display-p3)}\n"
        ),
        "a {\
         \n  b: color(srgb none 0.2018961688 0.3005594241);\
         \n}\n"
    );
            }
            mod same {
                use super::runner;

                #[test]
                #[ignore] // unexepected error
                fn explicit() {
                    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(color(srgb 0.1 0.2 0.3), $red: none, $space: srgb)}\n"
        ),
        "a {\
         \n  b: color(srgb none 0.2 0.3);\
         \n}\n"
    );
                }
                #[test]
                #[ignore] // unexepected error
                fn implicit() {
                    assert_eq!(
                        runner().ok("@use \"sass:color\";\
             \na {b: color.change(color(srgb 0.1 0.2 0.3), $red: none)}\n"),
                        "a {\
         \n  b: color(srgb none 0.2 0.3);\
         \n}\n"
                    );
                }
            }
        }
    }
    mod color {
        use super::runner;

        mod legacy {
            use super::runner;

            mod analogous {
                use super::runner;

                #[test]
                #[ignore] // unexepected error
                fn legacy() {
                    assert_eq!(
                        runner().ok("@use \"sass:color\";\
             \na {b: color.change(hsl(none 50% 50%), $space: hwb)}\n"),
                        "a {\
         \n  b: hsl(0, 50%, 50%);\
         \n}\n"
                    );
                }
                #[test]
                #[ignore] // unexepected error
                fn modern() {
                    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(rgb(none none none), $space: display-p3)}\n"
        ),
        "a {\
         \n  b: black;\
         \n}\n"
    );
                }
            }
            mod same {
                use super::runner;

                #[test]
                #[ignore] // unexepected error
                fn explicit() {
                    assert_eq!(
                        runner().ok("@use \"sass:color\";\
             \na {b: color.change(rgb(none none none), $space: rgb)}\n"),
                        "a {\
         \n  b: rgb(none none none);\
         \n}\n"
                    );
                }
                #[test]
                #[ignore] // unexepected error
                fn implicit() {
                    assert_eq!(
                        runner().ok("@use \"sass:color\";\
             \na {b: color.change(rgb(none none none))}\n"),
                        "a {\
         \n  b: rgb(none none none);\
         \n}\n"
                    );
                }
            }
        }
        mod modern {
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn analogous() {
                assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(color(srgb none none none), $space: display-p3)}\n"
        ),
        "a {\
         \n  b: color(srgb none none none);\
         \n}\n"
    );
            }
            mod same {
                use super::runner;

                #[test]
                #[ignore] // unexepected error
                fn explicit() {
                    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(color(srgb none none none), $space: srgb)}\n"
        ),
        "a {\
         \n  b: color(srgb none none none);\
         \n}\n"
    );
                }
                #[test]
                #[ignore] // unexepected error
                fn implicit() {
                    assert_eq!(
                        runner().ok("@use \"sass:color\";\
             \na {b: color.change(color(srgb none none none))}\n"),
                        "a {\
         \n  b: color(srgb none none none);\
         \n}\n"
                    );
                }
            }
        }
    }
}
mod modern {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn to_legacy() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(lab(50% 10 -20), $saturation: 5%, $space: hsl)}\n"
        ),
        "a {\
         \n  b: lab(53.9442763509% 2.9406893179 -6.1872703789);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn to_modern() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(lab(50% 10 -20), $chroma: 0.01, $space: oklch)}\n"
        ),
        "a {\
         \n  b: lab(50.5994458541% 1.4999074176 -3.4111169436);\
         \n}\n"
    );
    }
}
mod powerless {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn legacy() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(hsl(0deg 0% 50%), $space: lab)}\n"),
            "a {\
         \n  b: hsl(0, 0%, 50%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn modern() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(oklch(50% 0 0deg), $space: lab)}\n"),
            "a {\
         \n  b: oklch(50% 0 none);\
         \n}\n"
        );
    }
}
