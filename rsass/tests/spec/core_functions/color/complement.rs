//! Tests auto-converted from "sass-spec/spec/core_functions/color/complement.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("complement")
}

#[test]
fn alpha() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.complement(rgba(turquoise, 0.7))}\n"),
        "a {\
         \n  b: rgba(224, 64, 80, 0.7);\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;

    mod null_space {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn non_legacy() {
            assert_eq!(
                runner().err(
                    "@use \"sass:color\";\
             \na {b: color.complement(oklch(0.63 0.26 29.2))}\n"
                ),
                "Error: $space: null is not a string.\
         \n  ,\
         \n2 | a {b: color.complement(oklch(0.63 0.26 29.2))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
    mod space {
        #[allow(unused)]
        use super::runner;

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
                    #[ignore] // wrong error
                    fn hue() {
                        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.complement(hsl(none 30% 40%), $space: lch)}\n"
        ),
        "Error: $hue: Because the CSS working group is still deciding on the best behavior, Sass doesn\'t currently support modifying missing channels (color: lch(38.0910184332% 29.3078189694 none)).\
         \n  ,\
         \n2 | a {b: color.complement(hsl(none 30% 40%), $space: lch)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
                    }
                    #[test]
                    #[ignore] // wrong error
                    fn lightness() {
                        assert_eq!(
        runner().err(
            "// This is an error specifically because hsl(0deg 50% none) converts through\
             \n// rgb(0 0 0) to lch(none 0 none), so adjusting the hue fails.\
             \n@use \"sass:color\";\
             \na {b: color.complement(hsl(0deg 50% none), $space: lch)}\n"
        ),
        "Error: $hue: Because the CSS working group is still deciding on the best behavior, Sass doesn\'t currently support modifying missing channels (color: lch(none 0 none)).\
         \n  ,\
         \n4 | a {b: color.complement(hsl(0deg 50% none), $space: lch)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 4:7  root stylesheet",
    );
                    }
                }
                mod same {
                    #[allow(unused)]
                    use super::runner;

                    #[test]
                    #[ignore] // wrong error
                    fn explicit() {
                        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.complement(hsl(none 30% 40%), $space: hsl)}\n"
        ),
        "Error: $hue: Because the CSS working group is still deciding on the best behavior, Sass doesn\'t currently support modifying missing channels (color: hsl(none 30% 40%)).\
         \n  ,\
         \n2 | a {b: color.complement(hsl(none 30% 40%), $space: hsl)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
                    }
                    #[test]
                    #[ignore] // wrong error
                    fn implicit() {
                        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.complement(hsl(none 30% 40%))}\n"
        ),
        "Error: $hue: Because the CSS working group is still deciding on the best behavior, Sass doesn\'t currently support modifying missing channels (color: hsl(none 30% 40%)).\
         \n  ,\
         \n2 | a {b: color.complement(hsl(none 30% 40%))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
                    }
                }
            }
            mod modern {
                #[allow(unused)]
                use super::runner;

                #[test]
                #[ignore] // wrong error
                fn analogous() {
                    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.complement(lch(40% 30% none), $space: lch)}\n"
        ),
        "Error: $hue: Because the CSS working group is still deciding on the best behavior, Sass doesn\'t currently support modifying missing channels (color: lch(40% 45 none)).\
         \n  ,\
         \n2 | a {b: color.complement(lch(40% 30% none), $space: lch)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
                }
                #[test]
                #[ignore] // wrong error
                fn same() {
                    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.complement(lch(40% 30% none), $space: hsl)}\n"
        ),
        "Error: $hue: Because the CSS working group is still deciding on the best behavior, Sass doesn\'t currently support modifying missing channels (color: hsl(none 46.6772108151% 42.1546574074%)).\
         \n  ,\
         \n2 | a {b: color.complement(lch(40% 30% none), $space: hsl)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
                }
            }
        }
        #[test]
        #[ignore] // wrong error
        fn non_polar_angle() {
            assert_eq!(
                runner().err(
                    "@use \"sass:color\";\
             \na {b: color.complement(red, xyz)}\n"
                ),
                "Error: $space: Color space xyz doesn\'t have a hue channel.\
         \n  ,\
         \n2 | a {b: color.complement(red, xyz)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        mod powerless {
            #[allow(unused)]
            use super::runner;

            mod legacy {
                #[allow(unused)]
                use super::runner;

                #[test]
                #[ignore] // wrong error
                fn explicit() {
                    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.complement(grey, $space: hsl)}\n"
        ),
        "Error: $hue: Because the CSS working group is still deciding on the best behavior, Sass doesn\'t currently support modifying missing channels (color: hsl(none 0% 50.1960784314%)).\
         \n  ,\
         \n2 | a {b: color.complement(grey, $space: hsl)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
                }
            }
            #[test]
            #[ignore] // wrong error
            fn modern() {
                assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.complement(lab(50% 0 0), $space: lch)}\n"
        ),
        "Error: $hue: Because the CSS working group is still deciding on the best behavior, Sass doesn\'t currently support modifying missing channels (color: lch(50% 0 none)).\
         \n  ,\
         \n2 | a {b: color.complement(lab(50% 0 0), $space: lch)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
            }
        }
    }
    #[test]
    #[ignore] // wrong error
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.complement()}\n"
            ),
            "Error: Missing argument $color.\
         \n  ,--> input.scss\
         \n2 | a {b: color.complement()}\
         \n  |       ^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function complement($color, $space: null) {\
         \n  |           ================================ declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.complement(red, lch, 1)}\n"
            ),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: color.complement(red, lch, 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function complement($color, $space: null) {\
         \n  |           ================================ declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.complement(1)}\n"
            ),
            "Error: $color: 1 is not a color.\
         \n  ,\
         \n2 | a {b: color.complement(1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
#[ignore] // unexepected error
fn explicit_space() {
    assert_eq!(
        runner().ok("a {b: complement(red, hwb)}\n"),
        "a {\
         \n  b: aqua;\
         \n}\n"
    );
}
mod grayscale {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn black() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.complement(black)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    fn gray() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.complement(gray)}\n"),
            "a {\
         \n  b: gray;\
         \n}\n"
        );
    }
    #[test]
    fn white() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.complement(white)}\n"),
            "a {\
         \n  b: white;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.complement($color: red, $space: hwb)}\n"),
        "a {\
         \n  b: aqua;\
         \n}\n"
    );
}
#[test]
fn red() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.complement(red)}\n"),
        "a {\
         \n  b: aqua;\
         \n}\n"
    );
}
mod space {
    #[allow(unused)]
    use super::runner;

    mod legacy {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn to_legacy() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.complement(red, $space: hwb)}\n"),
                "a {\
         \n  b: aqua;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn to_modern() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.complement(red, $space: oklch)}\n"),
                "a {\
         \n  b: hsl(188.199882451, 488.180958059%, 14.5886916745%);\
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

            mod different {
                #[allow(unused)]
                use super::runner;

                #[test]
                #[ignore] // unexepected error
                fn explicit() {
                    assert_eq!(
                        runner().ok("@use \"sass:color\";\
             \na {b: color.complement(hwb(0deg 50% none), $space: hsl)}\n"),
                        "a {\
         \n  b: hsl(180, 100%, 75%);\
         \n}\n"
                    );
                }
                #[test]
                #[ignore] // unexepected error
                fn implicit() {
                    assert_eq!(
                        runner().ok("@use \"sass:color\";\
             \na {b: color.complement(hwb(0deg 50% none))}\n"),
                        "a {\
         \n  b: hsl(180, 100%, 75%);\
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
             \na {b: color.complement(hsl(0deg 50% none), $space: hsl)}\n"),
                        "a {\
         \n  b: hsl(180deg 50% none);\
         \n}\n"
                    );
                }
                #[test]
                #[ignore] // unexepected error
                fn implicit() {
                    assert_eq!(
                        runner().ok("@use \"sass:color\";\
             \na {b: color.complement(hsl(0deg 50% none))}\n"),
                        "a {\
         \n  b: hsl(180deg 50% none);\
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
                    runner().ok("@use \"sass:color\";\
             \na {b: color.complement(lch(none 50% 0deg), $space: oklch)}\n"),
                    "a {\
         \n  b: lch(none 24.5072331187 180.9107983391deg);\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn different() {
                assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.complement(color(srgb none 100 200), $space: oklch)}\n"
        ),
        "a {\
         \n  b: color(srgb 156.4212742119 85.1578604941 -71.226404054);\
         \n}\n"
    );
            }
            #[test]
            #[ignore] // unexepected error
            fn same() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \na {b: color.complement(lch(none 50% 0deg), $space: hsl)}\n"),
                    "a {\
         \n  b: lch(none 0 none);\
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
        fn to_legacy() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.complement(lab(50% 10 -20), $space: hsl)}\n"),
                "a {\
         \n  b: lab(61.7521821385% -8.5219772697 19.4842183485);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn to_modern() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.complement(lab(50% 10 -20), $space: oklch)}\n"),
                "a {\
         \n  b: lab(51.226961501% -7.7080869407 21.8652805706);\
         \n}\n"
            );
        }
    }
    mod powerless {
        #[allow(unused)]
        use super::runner;

        mod legacy {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn implicit() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \na {b: color.complement(hsl(0deg 0% 50%))}\n"),
                    "a {\
         \n  b: hsl(180, 0%, 50%);\
         \n}\n"
                );
            }
        }
    }
}
#[test]
fn turquoise() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.complement(turquoise)}\n"),
        "a {\
         \n  b: #e04050;\
         \n}\n"
    );
}
