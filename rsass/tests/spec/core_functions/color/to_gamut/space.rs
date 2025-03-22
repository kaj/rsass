//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_gamut/space.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("space")
}

mod clip {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn in_gamut() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(oklch(60% 0.13 240deg), $space: rgb, $method: clip)}\n"
        ),
        "a {\
         \n  b: oklch(60% 0.13 240deg);\
         \n}\n"
    );
    }
    mod missing {
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
             \na {b: color.to-gamut(hsl(none 50% 50%), $space: hwb, $method: clip)}\n"
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
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(rgb(none none none), $space: display-p3, $method: clip)}\n"
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
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(rgb(none none none), $space: rgb, $method: clip)}\n"
        ),
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
             \na {b: color.to-gamut(rgb(none none none), $method: clip)}\n"),
                        "a {\
         \n  b: rgb(none none none);\
         \n}\n"
                    );
                }
                #[test]
                #[ignore] // unexepected error
                fn with_mapping() {
                    assert_eq!(
                        runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.to-gamut(\
             \n    color.change(rgb(none 0 none), $green: 300),\
             \n    $method: clip\
             \n  );\
             \n}\n"),
                        "a {\
         \n  b: rgb(none 255 none);\
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
                    runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.to-gamut(\
             \n    color(srgb none none none),\
             \n    $space: display-p3,\
             \n    $method: clip\
             \n  );\
             \n}\n"),
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
             \na {b: color.to-gamut(color(srgb none none none), $space: srgb, $method: clip)}\n"
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
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(srgb none none none), $method: clip)}\n"
        ),
        "a {\
         \n  b: color(srgb none none none);\
         \n}\n"
    );
                }
                #[test]
                #[ignore] // unexepected error
                fn with_mapping() {
                    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(srgb none 1.2 none), $method: clip)}\n"
        ),
        "a {\
         \n  b: color(srgb none 1 none);\
         \n}\n"
    );
                }
            }
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_gamut() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(oklch(60% 0.15 240deg), $space: rgb, $method: clip)}\n"
        ),
        "a {\
         \n  b: oklch(60.4068471201% 0.1443906236 242.3895367243deg);\
         \n}\n"
    );
    }
    mod powerless {
        use super::runner;

        mod legacy {
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn no_mapping() {
                assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(hsl(0deg 0% 20%), $space: srgb, $method: clip)}\n"
        ),
        "a {\
         \n  b: hsl(0, 0%, 20%);\
         \n}\n"
    );
            }
            #[test]
            #[ignore] // unexepected error
            fn with_mapping() {
                assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(hsl(0deg 10% 1000%), $space: srgb, $method: clip)}\n"
        ),
        "a {\
         \n  b: hsl(0, 0%, 100%);\
         \n}\n"
    );
            }
        }
        mod modern {
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn no_mapping() {
                assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(oklch(10% 0% 0deg), $space: srgb, $method: clip)}\n"
        ),
        "a {\
         \n  b: oklch(10% 0 none);\
         \n}\n"
    );
            }
            #[test]
            #[ignore] // unexepected error
            fn with_mapping() {
                assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(lch(1000% 10% 0deg), $space: hsl, $method: clip)}\n"
        ),
        "a {\
         \n  b: lch(100% 0 none);\
         \n}\n"
    );
            }
        }
    }
}
mod local_minde {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn in_gamut() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(oklch(60% 0.13 240deg), $space: rgb, $method: local-minde)}\n"
        ),
        "a {\
         \n  b: oklch(60% 0.13 240deg);\
         \n}\n"
    );
    }
    mod missing {
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
             \na {b: color.to-gamut(hsl(none 50% 50%), $space: hwb, $method: local-minde)}\n"
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
             \na {\
             \n  b: color.to-gamut(\
             \n    rgb(none none none),\
             \n    $space: display-p3,\
             \n    $method: local-minde\
             \n  );\
             \n}\n"),
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
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(rgb(none none none), $space: rgb, $method: local-minde)}\n"
        ),
        "a {\
         \n  b: rgb(none none none);\
         \n}\n"
    );
                }
                #[test]
                #[ignore] // unexepected error
                fn implicit() {
                    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(rgb(none none none), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: rgb(none none none);\
         \n}\n"
    );
                }
                #[test]
                #[ignore] // unexepected error
                fn with_mapping() {
                    assert_eq!(
                        runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.to-gamut(\
             \n    color.change(rgb(none 0 none), $green: 300),\
             \n    $method: local-minde\
             \n  );\
             \n}\n"),
                        "a {\
         \n  b: rgb(220.0211077797, 255, 215.6454381401);\
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
                    runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.to-gamut(\
             \n    color(srgb none none none),\
             \n    $space: display-p3,\
             \n    $method: local-minde\
             \n  );\
             \n}\n"),
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
                        runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.to-gamut(\
             \n    color(srgb none none none),\
             \n    $space: srgb,\
             \n    $method: local-minde\
             \n  );\
             \n}\n"),
                        "a {\
         \n  b: color(srgb none none none);\
         \n}\n"
                    );
                }
                #[test]
                #[ignore] // unexepected error
                fn implicit() {
                    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(srgb none none none), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: color(srgb none none none);\
         \n}\n"
    );
                }
                #[test]
                #[ignore] // unexepected error
                fn with_mapping() {
                    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(srgb none 1.2 none), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: color(srgb 0.9249489701 1 0.9135715783);\
         \n}\n"
    );
                }
            }
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_gamut() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(oklch(60% 0.15 240deg), $space: rgb, $method: local-minde)}\n"
        ),
        "a {\
         \n  b: oklch(60.4068471201% 0.1443906236 242.3895367243deg);\
         \n}\n"
    );
    }
    mod powerless {
        use super::runner;

        mod legacy {
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn no_mapping() {
                assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(hsl(0deg 0% 20%), $space: srgb, $method: local-minde)}\n"
        ),
        "a {\
         \n  b: hsl(0, 0%, 20%);\
         \n}\n"
    );
            }
            #[test]
            #[ignore] // unexepected error
            fn with_mapping() {
                assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(hsl(0deg 10% 1000%), $space: srgb, $method: local-minde)}\n"
        ),
        "a {\
         \n  b: hsl(0, 0%, 100%);\
         \n}\n"
    );
            }
        }
        mod modern {
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn no_mapping() {
                assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(oklch(10% 0% 0deg), $space: srgb, $method: local-minde)}\n"
        ),
        "a {\
         \n  b: oklch(10% 0 none);\
         \n}\n"
    );
            }
            #[test]
            #[ignore] // unexepected error
            fn with_mapping() {
                assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(lch(1000% 10% 0deg), $space: srgb, $method: local-minde)}\n"
        ),
        "a {\
         \n  b: lch(100% 0 none);\
         \n}\n"
    );
            }
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn to_black() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(oklch(0% 0.13 240deg), $space: rgb, $method: local-minde)}\n"
        ),
        "a {\
         \n  b: oklch(0% 0 none);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn to_white() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(oklch(100% 0.13 240deg), $space: rgb, $method: local-minde)}\n"
        ),
        "a {\
         \n  b: oklch(100% 0 none);\
         \n}\n"
    );
    }
}
