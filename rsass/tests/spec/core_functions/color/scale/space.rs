//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/space.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("space")
}

mod legacy {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn to_legacy() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(red, $saturation: -10%, $space: hsl)}\n"),
            "a {\
         \n  b: rgb(242.25, 12.75, 12.75);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn to_modern() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(red, $a: 10%, $space: lab)}\n"),
            "a {\
         \n  b: hsl(352.5777091359, 128.657405446%, 44.6392534152%);\
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
             \na {b: color.scale(hsl(none 50% 50%), $space: hwb)}\n"),
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
             \na {b: color.scale(rgb(none none none), $space: display-p3)}\n"
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
             \na {b: color.scale(rgb(none none none), $space: rgb)}\n"),
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
             \na {b: color.scale(rgb(none none none))}\n"),
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
             \na {b: color.scale(color(srgb none none none), $space: display-p3)}\n"
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
             \na {b: color.scale(color(srgb none none none), $space: srgb)}\n"
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
             \na {b: color.scale(color(srgb none none none))}\n"),
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
             \na {b: color.scale(lab(50% 10 -20), $saturation: -20%, $space: hsl)}\n"
        ),
        "a {\
         \n  b: lab(51.1280465895% 7.8762091679 -15.9907381545);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn to_modern() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(lab(50% 10 -20), $chroma: 20%, $space: oklch)}\n"
        ),
        "a {\
         \n  b: lab(48.9487510552% 24.2085116419 -41.7322265307);\
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
             \na {b: color.scale(hsl(0deg 0% 50%), $space: lab)}\n"),
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
             \na {b: color.scale(oklch(50% 0 0deg), $space: lab)}\n"),
            "a {\
         \n  b: oklch(50% 0 none);\
         \n}\n"
        );
    }
}
