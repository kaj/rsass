//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust/space.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("space")
}

#[test]
#[ignore] // unexepected error
fn identical() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(oklch(50% 0.2 0deg), $space: lab)}\n"),
        "a {\
         \n  b: oklch(50% 0.2 0deg);\
         \n}\n"
    );
}
mod legacy {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn to_legacy() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(pink, $saturation: -5%, $space: hsl)}\n"),
            "a {\
         \n  b: rgb(253.425, 193.575, 204.025);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn to_modern() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(pink, $chroma: -0.01, $space: oklch)}\n"),
            "a {\
         \n  b: rgb(249.5073873455, 194.8272088475, 204.1290090118);\
         \n}\n"
        );
    }
}
mod missing {
    #[allow(unused)]
    use super::runner;

    mod legacy {
        #[allow(unused)]
        use super::runner;

        mod analogous {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn legacy() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(hsl(none 50% 50%), $space: hwb)}\n"),
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
             \na {b: color.adjust(rgb(none none none), $space: display-p3)}\n"
        ),
        "a {\
         \n  b: black;\
         \n}\n"
    );
            }
        }
        mod same {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn explicit() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(rgb(none none none), $space: rgb)}\n"),
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
             \na {b: color.adjust(rgb(none none none))}\n"),
                    "a {\
         \n  b: rgb(none none none);\
         \n}\n"
                );
            }
        }
    }
    mod modern {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn analogous() {
            assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(color(srgb none none none), $space: display-p3)}\n"
        ),
        "a {\
         \n  b: color(srgb none none none);\
         \n}\n"
    );
        }
        mod same {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn explicit() {
                assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(color(srgb none none none), $space: srgb)}\n"
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
             \na {b: color.adjust(color(srgb none none none))}\n"),
                    "a {\
         \n  b: color(srgb none none none);\
         \n}\n"
                );
            }
        }
    }
}
mod modern {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn to_legacy() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(lab(50% 10 -20), $saturation: 5%, $space: hsl)}\n"
        ),
        "a {\
         \n  b: lab(48.2797960499% 13.4192461856 -26.2119638245);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn to_modern() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(lab(50% 10 -20), $chroma: 0.01, $space: oklch)}\n"
        ),
        "a {\
         \n  b: lab(49.8635566117% 11.8934236813 -23.2342925555);\
         \n}\n"
    );
    }
}
mod powerless {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn legacy() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(hsl(0deg 0% 50%), $space: lab)}\n"),
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
             \na {b: color.adjust(oklch(50% 0 0deg), $space: lab)}\n"),
            "a {\
         \n  b: oklch(50% 0 none);\
         \n}\n"
        );
    }
}
