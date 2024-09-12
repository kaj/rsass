//! Tests auto-converted from "sass-spec/spec/core_functions/color/mix/missing.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("missing")
}

mod explicit {
    #[allow(unused)]
    use super::runner;

    mod analogous {
        #[allow(unused)]
        use super::runner;

        mod legacy {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn both() {
                assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(rgb(0 none 200), rgb(200 none 0), $method: rec2020)}\n"
        ),
        "a {\
         \n  b: rgb(129.0248146672, 0, 115.9531222724);\
         \n}\n"
    );
            }
            #[test]
            #[ignore] // unexepected error
            fn color1() {
                assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(rgb(none 100 200), rgb(200 100 0), $method: rec2020)}\n"
        ),
        "a {\
         \n  b: rgb(199.7671172587, 91.8239078594, 117.7284104313);\
         \n}\n"
    );
            }
            #[test]
            #[ignore] // unexepected error
            fn color2() {
                assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(rgb(0 100 200), rgb(200 none 0), $method: rec2020)}\n"
        ),
        "a {\
         \n  b: rgb(128.6114294932, 95.678836923, 112.8090426091);\
         \n}\n"
    );
            }
        }
        mod modern {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn both() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.mix(\
             \n    color(srgb 0.1 0.2 none),\
             \n    color(srgb 0.3 0.2 none),\
             \n    $method: rec2020\
             \n  );\
             \n}\n"),
                    "a {\
         \n  b: color(srgb 0.2110804397 0.2012136757 none);\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn color1() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.mix(\
             \n    color(srgb none 0.1 0.2),\
             \n    color(srgb 0.1 0.2 0.3),\
             \n    $method: rec2020\
             \n  );\
             \n}\n"),
                    "a {\
         \n  b: color(srgb 0.146201736 0.1492595622 0.2497768167);\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn color2() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.mix(\
             \n    color(srgb 0.1 0.2 0.3),\
             \n    color(srgb 0.1 none 0.2),\
             \n    $method: rec2020\
             \n  );\
             \n}\n"),
                    "a {\
         \n  b: color(srgb 0.0134568276 0.2030946777 0.2456679982);\
         \n}\n"
                );
            }
        }
    }
    mod same {
        #[allow(unused)]
        use super::runner;

        mod legacy {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn both() {
                assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(rgb(0 none 200), rgb(200 none 0), $method: rgb)}\n"
        ),
        "a {\
         \n  b: rgb(100 none 100);\
         \n}\n"
    );
            }
            #[test]
            #[ignore] // unexepected error
            fn color1() {
                assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(rgb(none 100 200), rgb(200 100 0), $method: rgb)}\n"
        ),
        "a {\
         \n  b: #c86464;\
         \n}\n"
    );
            }
            #[test]
            #[ignore] // unexepected error
            fn color2() {
                assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(rgb(0 100 200), rgb(200 none 0), $method: rgb)}\n"
        ),
        "a {\
         \n  b: #646464;\
         \n}\n"
    );
            }
        }
        mod modern {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn both() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.mix(\
             \n    color(srgb 0.1 0.2 none),\
             \n    color(srgb 0.3 0.2 none),\
             \n    $method: srgb\
             \n  );\
             \n}\n"),
                    "a {\
         \n  b: color(srgb 0.2 0.2 none);\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn color1() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.mix(\
             \n    color(srgb none 0.1 0.2),\
             \n    color(srgb 0.1 0.2 0.3),\
             \n    $method: srgb\
             \n  );\
             \n}\n"),
                    "a {\
         \n  b: color(srgb 0.1 0.15 0.25);\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn color2() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.mix(\
             \n    color(srgb 0.1 0.2 0.3),\
             \n    color(srgb 0.1 none 0.2),\
             \n    $method: srgb\
             \n  );\
             \n}\n"),
                    "a {\
         \n  b: color(srgb 0.1 0.2 0.25);\
         \n}\n"
                );
            }
        }
    }
}
mod powerless {
    #[allow(unused)]
    use super::runner;

    mod legacy {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn both() {
            assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(hsl(120deg 0% 50%), hsl(0deg 0% 30%), $method: lch)}\n"
        ),
        "a {\
         \n  b: hsl(0, 0%, 39.7779408276%);\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn color1() {
            assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(hsl(0deg 0% 30%), hsl(120deg 50% 50%), $method: lch)}\n"
        ),
        "a {\
         \n  b: hsl(113.4583259264, 28.061366187%, 40.5877359835%);\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn color2() {
            assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(hsl(120deg 50% 50%), hsl(0deg 0% 30%), $method: lch)}\n"
        ),
        "a {\
         \n  b: hsl(113.4583259264, 28.061366187%, 40.5877359835%);\
         \n}\n"
    );
        }
    }
    mod modern {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn both() {
            assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(lch(50% 0% 120deg), lch(30% 0% 0deg), $method: hsl)}\n"
        ),
        "a {\
         \n  b: lch(40.2238896861% 0 none);\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn color1() {
            assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(lch(30% 0% 0deg), lch(50% 10% 120deg), $method: hsl)}\n"
        ),
        "a {\
         \n  b: lch(39.8551054023% 6.455971398 120.4338354849deg);\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn color2() {
            assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(lch(50% 10% 120deg), lch(30% 0% 0deg), $method: hsl)}\n"
        ),
        "a {\
         \n  b: lch(39.8551054023% 6.455971398 120.4338354849deg);\
         \n}\n"
    );
        }
    }
}
