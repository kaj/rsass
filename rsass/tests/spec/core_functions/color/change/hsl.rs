//! Tests auto-converted from "sass-spec/spec/core_functions/color/change/hsl.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hsl")
}

#[test]
#[ignore] // wrong result
fn all() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(black, $hue: 12, $saturation: 24%, $lightness: 48%)}\n"
        ),
        "a {\
         \n  b: rgb(151.776, 104.7744, 93.024);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn alpha_arg() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(black, $hue: 12, $saturation: 24%, $lightness: 48%, $alpha: 0.7)}\n"
        ),
        "a {\
         \n  b: rgba(151.776, 104.7744, 93.024, 0.7);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn alpha_input() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(rgba(black, 0.7), $hue: 12, $saturation: 24%, $lightness: 48%)}\n"
        ),
        "a {\
         \n  b: rgba(151.776, 104.7744, 93.024, 0.7);\
         \n}\n"
    );
}
mod hue {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(red, $hue: 540)}\n"),
            "a {\
         \n  b: aqua;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn fraction() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(red, $hue: 0.5)}\n"),
            "a {\
         \n  b: rgb(255, 2.125, 0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(red, $hue: 359)}\n"),
            "a {\
         \n  b: rgb(255, 0, 4.25);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn middle() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(red, $hue: 123)}\n"),
            "a {\
         \n  b: rgb(0, 255, 12.75);\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(blue, $hue: 0)}\n"),
            "a {\
         \n  b: red;\
         \n}\n"
        );
    }
    #[test]
    fn negative() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(red, $hue: -60)}\n"),
            "a {\
         \n  b: fuchsia;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn none() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(hsl(0deg 100% 50%), $hue: none)}\n"),
            "a {\
         \n  b: hsl(none 100% 50%);\
         \n}\n"
        );
    }
}
mod lightness {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn above_range() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(red, $lightness: 120%)}\n"),
            "a {\
         \n  b: hsl(0, 100%, 120%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn below_range() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(red, $lightness: -20%)}\n"),
            "a {\
         \n  b: hsl(0, 100%, -20%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn fraction() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(red, $lightness: 0.5%)}\n"),
            "a {\
         \n  b: rgb(2.55, 0, 0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(red, $lightness: 63%)}\n"),
            "a {\
         \n  b: rgb(255, 66.3, 66.3);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(red, $lightness: 27%)}\n"),
            "a {\
         \n  b: rgb(137.7, 0, 0);\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(red, $lightness: 100%)}\n"),
            "a {\
         \n  b: white;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(red, $lightness: 0%)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn none() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(hsl(0deg 100% 50%), $lightness: none)}\n"),
            "a {\
         \n  b: hsl(0deg 100% none);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // wrong result
fn named() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change($color: black, $hue: 12, $saturation: 24%, $lightness: 48%)}\n"
        ),
        "a {\
         \n  b: rgb(151.776, 104.7744, 93.024);\
         \n}\n"
    );
}
mod saturation {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn above_range() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(plum, $saturation: 120%)}\n"),
            "a {\
         \n  b: hsl(300, 120%, 74.7058823529%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn below_range() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(plum, $saturation: -20%)}\n"),
            "a {\
         \n  b: rgb(177.6, 203.4, 177.6);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(plum, $saturation: 76%)}\n"),
            "a {\
         \n  b: rgb(239.52, 141.48, 239.52);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(plum, $saturation: 14%)}\n"),
            "a {\
         \n  b: rgb(199.53, 181.47, 199.53);\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(plum, $saturation: 100%)}\n"),
            "a {\
         \n  b: #ff7eff;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(plum, $saturation: 0%)}\n"),
            "a {\
         \n  b: rgb(190.5, 190.5, 190.5);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn none() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(hsl(0deg 100% 50%), $saturation: none)}\n"),
            "a {\
         \n  b: hsl(0deg none 50%);\
         \n}\n"
        );
    }
}
mod units {
    #[allow(unused)]
    use super::runner;

    mod hue {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn angle() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.change(red, $hue: 60rad)}\n"),
                "a {\
         \n  b: rgb(0, 179.576224164, 255);\
         \n}\n"
            );
        }
        #[test]
        fn deg() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.change(red, $hue: 60deg)}\n"),
                "a {\
         \n  b: yellow;\
         \n}\n"
            );
        }
        #[test]
        fn unitless() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.change(red, $hue: 60)}\n"),
                "a {\
         \n  b: yellow;\
         \n}\n"
            );
        }
        #[test]
        fn unknown() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.change(red, $hue: 60in)}\n"),
                "a {\
         \n  b: yellow;\
         \n}\n"
            );
        }
    }
    mod lightness {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn percent() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.change(red, $lightness: 30%)}\n"),
                "a {\
         \n  b: #990000;\
         \n}\n"
            );
        }
        #[test]
        fn unitless() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.change(red, $lightness: 30)}\n"),
                "a {\
         \n  b: #990000;\
         \n}\n"
            );
        }
        #[test]
        fn unknown() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.change(red, $lightness: 30in)}\n"),
                "a {\
         \n  b: #990000;\
         \n}\n"
            );
        }
    }
    mod saturation {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn percent() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.change(red, $saturation: 50%)}\n"),
                "a {\
         \n  b: rgb(191.25, 63.75, 63.75);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn unitless() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.change(red, $saturation: 50)}\n"),
                "a {\
         \n  b: rgb(191.25, 63.75, 63.75);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn unknown() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.change(red, $saturation: 50in)}\n"),
                "a {\
         \n  b: rgb(191.25, 63.75, 63.75);\
         \n}\n"
            );
        }
    }
}
