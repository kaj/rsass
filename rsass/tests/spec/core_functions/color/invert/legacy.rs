//! Tests auto-converted from "sass-spec/spec/core_functions/color/invert/legacy.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("legacy")
}

mod no_space {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn black() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.invert(black)}\n"),
            "a {\
         \n  b: white;\
         \n}\n"
        );
    }
    #[test]
    fn gray() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.invert(gray)}\n"),
            "a {\
         \n  b: #7f7f7f;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn hsl() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.invert(hsl(30deg 20% 40%))}\n"),
            "a {\
         \n  b: hsl(210, 20%, 60%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn hwb() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.invert(hsl(30deg 20% 40%))}\n"),
            "a {\
         \n  b: hsl(210, 20%, 60%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_gamut() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(\
             \n  color.invert(color.change(rgb(0 50 0), $red: -100, $blue: 500))\
             \n);\n"
        ),
        "a {\
         \n  value: hsl(45, 545.4545454545%, 21.568627451%);\
         \n  space: rgb;\
         \n  channels: 355 205 -245 / 1;\
         \n}\n"
    );
    }
    #[test]
    fn turquoise() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.invert(turquoise)}\n"),
            "a {\
         \n  b: #bf1f2f;\
         \n}\n"
        );
    }
    mod weighted {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn high() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.invert(turquoise, 92%)}\n"),
                "a {\
         \n  b: rgb(180.84, 46.44, 59.88);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn low() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.invert(turquoise, 23%)}\n"),
                "a {\
         \n  b: rgb(93.21, 179.61, 170.97);\
         \n}\n"
            );
        }
        #[test]
        fn max() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.invert(turquoise, 100%)}\n"),
                "a {\
         \n  b: #bf1f2f;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn middle() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.invert(turquoise, 50%)}\n"),
                "a {\
         \n  b: rgb(127.5, 127.5, 127.5);\
         \n}\n"
            );
        }
        #[test]
        fn min() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.invert(turquoise, 0%)}\n"),
                "a {\
         \n  b: turquoise;\
         \n}\n"
            );
        }
    }
    #[test]
    fn white() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.invert(white)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
}
mod space {
    #[allow(unused)]
    use super::runner;

    mod hsl {
        #[allow(unused)]
        use super::runner;

        mod missing {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong error
            fn analogous() {
                assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.invert(hsl(30deg none 40%), $space: lch)}\n"
        ),
        "Error: $hue: Because the CSS working group is still deciding on the best behavior, Sass doesn\'t currently support modifying missing channels (color: lch(43.192289563% none none)).\
         \n  ,\
         \n2 | a {b: color.invert(hsl(30deg none 40%), $space: lch)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
            }
            #[test]
            #[ignore] // unexepected error
            fn same() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \na {b: color.invert(hsl(30deg none 40%), $space: hsl)}\n"),
                    "a {\
         \n  b: hsl(210deg none 60%);\
         \n}\n"
                );
            }
        }
        #[test]
        #[ignore] // unexepected error
        fn no_missing() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.invert(hsl(30deg 20% 40%), $space: hsl)}\n"),
                "a {\
         \n  b: hsl(210, 20%, 60%);\
         \n}\n"
            );
        }
    }
    mod hwb {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn missing() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.invert(hwb(30deg none 40%), $space: hwb)}\n"),
                "a {\
         \n  b: hwb(210deg 40% none);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn no_missing() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.invert(hwb(30deg 20% 40%), $space: hwb)}\n"),
                "a {\
         \n  b: #6699cc;\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn modern() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.invert(#abcdef, $space: display-p3)}\n"),
            "a {\
         \n  b: rgb(81.948808543, 49.5520621504, 10.5024231747);\
         \n}\n"
        );
    }
    mod powerless {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn different() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.invert(hsl(120deg 0% 40%), $space: rgb)}\n"),
                "a {\
         \n  b: hsl(0, 0%, 60%);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn same() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.invert(hsl(120deg 0% 40%), $space: hsl)}\n"),
                "a {\
         \n  b: hsl(300, 0%, 60%);\
         \n}\n"
            );
        }
    }
}
mod units {
    #[allow(unused)]
    use super::runner;

    mod weight {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn unitless() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.invert(turquoise, 10)}\n"),
                "a {\
         \n  b: rgb(76.7, 204.7, 191.9);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn unknown() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.invert(turquoise, 10px)}\n"),
                "a {\
         \n  b: rgb(76.7, 204.7, 191.9);\
         \n}\n"
            );
        }
    }
}
