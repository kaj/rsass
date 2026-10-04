//! Tests auto-converted from "sass-spec/spec/core_functions/color/mix/missing.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("missing")
}

mod explicit {
    use super::runner;

    mod analogous {
        use super::runner;

        mod legacy {
            use super::runner;

            mod both {
                use super::runner;

                #[test]
                #[ignore] // unexepected error
                fn all() {
                    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(rgb(none none none), rgb(none none none), $method: rec2020)}\n"
        ),
        "a {\
         \n  b: black;\
         \n}\n"
    );
                }
                #[test]
                #[ignore] // unexepected error
                fn set() {
                    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(hsl(none none 50%), lab(80% none none), $method: rec2020)}\n"
        ),
        "a {\
         \n  b: hsl(0, 0%, 63.8879192621%);\
         \n}\n"
    );
                }
                #[test]
                #[ignore] // unexepected error
                fn single() {
                    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(rgb(0 none 200), rgb(200 none 0), $method: rec2020)}\n"
        ),
        "a {\
         \n  b: rgb(50.0525424686%, 0%, 44.1197051136%);\
         \n}\n"
    );
                }
            }
            mod color1 {
                use super::runner;

                #[test]
                #[ignore] // unexepected error
                fn all() {
                    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(rgb(none none none), rgb(200 100 0), $method: rec2020)}\n"
        ),
        "a {\
         \n  b: #c86400;\
         \n}\n"
    );
                }
                #[test]
                #[ignore] // unexepected error
                fn set() {
                    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(hsl(none none 50%), lab(80% 10% 20%), $method: oklch)}\n"
        ),
        "a {\
         \n  b: hsl(27.6264349345, 39.5108871342%, 61.2332568327%);\
         \n}\n"
    );
                }
                #[test]
                #[ignore] // unexepected error
                fn single() {
                    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(rgb(none 100 200), rgb(200 100 0), $method: rec2020)}\n"
        ),
        "a {\
         \n  b: rgb(78.3685527456%, 36.0116672853%, 45.3323453854%);\
         \n}\n"
    );
                }
            }
            mod color2 {
                use super::runner;

                #[test]
                #[ignore] // unexepected error
                fn all() {
                    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(rgb(200 100 0), rgb(none none none), $method: rec2020)}\n"
        ),
        "a {\
         \n  b: #c86400;\
         \n}\n"
    );
                }
                #[test]
                #[ignore] // unexepected error
                fn set() {
                    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(hsl(120deg 10% 20%), lab(50% none none), $method: oklch)}\n"
        ),
        "a {\
         \n  b: hsl(119.669130857, 6.6488786052%, 32.4911149973%);\
         \n}\n"
    );
                }
                #[test]
                #[ignore] // unexepected error
                fn single() {
                    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(rgb(0 100 200), rgb(200 none 0), $method: rec2020)}\n"
        ),
        "a {\
         \n  b: rgb(50.1551278959%, 37.5727310145%, 42.8027026523%);\
         \n}\n"
    );
                }
            }
        }
        mod modern {
            use super::runner;

            mod both {
                use super::runner;

                #[test]
                #[ignore] // unexepected error
                fn all() {
                    assert_eq!(
                        runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.mix(\
             \n    color(srgb none none none),\
             \n    color(srgb none none none),\
             \n    $method: rec2020\
             \n  );\
             \n}\n"),
                        "a {\
         \n  b: color(srgb none none none);\
         \n}\n"
                    );
                }
                #[test]
                #[ignore] // unexepected error
                fn set() {
                    assert_eq!(
                        runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.mix(\
             \n    lab(20% none none),\
             \n    lch(50% none none),\
             \n    $method: oklch\
             \n  );\
             \n}\n"),
                        "a {\
         \n  b: lab(35% none none);\
         \n}\n"
                    );
                }
                #[test]
                #[ignore] // unexepected error
                fn single() {
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
         \n  b: color(srgb 0.210063151 0.2012856032 none);\
         \n}\n"
                    );
                }
            }
            mod color1 {
                use super::runner;

                #[test]
                #[ignore] // unexepected error
                fn all() {
                    assert_eq!(
                        runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.mix(\
             \n    color(srgb none none none),\
             \n    color(srgb 0.1 0.2 0.3),\
             \n    $method: rec2020\
             \n  );\
             \n}\n"),
                        "a {\
         \n  b: color(srgb 0.1 0.2 0.3);\
         \n}\n"
                    );
                }
                #[test]
                #[ignore] // unexepected error
                fn set() {
                    assert_eq!(
                        runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.mix(\
             \n    lch(50% none none),\
             \n    lab(80% 10% 20%),\
             \n    $method: oklch\
             \n  );\
             \n}\n"),
                        "a {\
         \n  b: lch(64.9034695294% 28.3412220859 63.2701468478deg);\
         \n}\n"
                    );
                }
                #[test]
                #[ignore] // unexepected error
                fn single() {
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
         \n  b: color(srgb 0.1485286314 0.1448485586 0.2496395015);\
         \n}\n"
                    );
                }
            }
            mod color2 {
                use super::runner;

                #[test]
                #[ignore] // unexepected error
                fn all() {
                    assert_eq!(
                        runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.mix(\
             \n    color(srgb 0.1 0.2 0.3),\
             \n    color(srgb none none none),\
             \n    $method: rec2020\
             \n  );\
             \n}\n"),
                        "a {\
         \n  b: color(srgb 0.1 0.2 0.3);\
         \n}\n"
                    );
                }
                #[test]
                #[ignore] // unexepected error
                fn set() {
                    assert_eq!(
                        runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.mix(\
             \n    lab(80% 10% 20%),\
             \n    lch(50% none none),\
             \n    $method: oklch\
             \n  );\
             \n}\n"),
                        "a {\
         \n  b: lab(64.9034695294% 12.7474400269 25.3125984854);\
         \n}\n"
                    );
                }
                #[test]
                #[ignore] // unexepected error
                fn single() {
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
         \n  b: color(srgb -0.0044280937 0.2034278916 0.2453243175);\
         \n}\n"
                    );
                }
            }
        }
    }
    mod same {
        use super::runner;

        mod legacy {
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
    use super::runner;

    mod legacy {
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
