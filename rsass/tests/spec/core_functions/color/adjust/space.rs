//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust/space.hrx"

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
         \n  b: rgb(249.5073881917, 194.8272088582, 204.1290059224);\
         \n}\n"
        );
    }
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
         \n  b: lab(49.8635566292% 11.8934232635 -23.234292765);\
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
