//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/one_arg.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("one_arg")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    mod clamped {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn above() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0 30% 40% / 1.1)}\n"),
                "a {\
         \n  b: hsl(0, 33.3333333333%, 45%);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn below() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0 30% 40% / -0.1)}\n"),
                "a {\
         \n  b: hsla(0, 33.3333333333%, 45%, 0);\
         \n}\n"
            );
        }
    }
    mod in_gamut {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn named() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb($channels: 180 30% 40% / 0.4)}\n"),
                "a {\
         \n  b: hsla(180, 33.3333333333%, 45%, 0.4);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn opaque() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(180 30% 40% / 1)}\n"),
                "a {\
         \n  b: hsl(180, 33.3333333333%, 45%);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn parenthesized() {
            assert_eq!(
        runner().ok(
            "@use \"sass:color\";\n\
             \n// Extra parens shouldn\'t cause the slash to be forced into division.\
             \na {b: (color.hwb(180 30% 40% / 0.4))}\n"
        ),
        "a {\
         \n  b: hsla(180, 33.3333333333%, 45%, 0.4);\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // wrong result
        fn partial() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(180 30% 40% / 0.5)}\n"),
                "a {\
         \n  b: hsla(180, 33.3333333333%, 45%, 0.5);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn transparent() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(180 30% 40% / 0)}\n"),
                "a {\
         \n  b: hsla(180, 33.3333333333%, 45%, 0);\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn var() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0 0 0 / var(--c))}\n"),
            "a {\
         \n  b: hwb(0 0 0/var(--c));\
         \n}\n"
        );
    }
}
mod blackness {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0 30% 101%)}\n"),
            "a {\
         \n  b: hsl(0, 0%, 22.9007633588%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0 30% -1%)}\n"),
            "a {\
         \n  b: hsl(0, 102.8985507246%, 65.5%);\
         \n}\n"
        );
    }
    mod var {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn alpha() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0 30% var(--c) / 0.5)}\n"),
                "a {\
         \n  b: hwb(0 30% var(--c)/0.5);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn no_alpha() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0 30% var(--c))}\n"),
                "a {\
         \n  b: hwb(0 30% var(--c));\
         \n}\n"
            );
        }
    }
}
mod hue {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn var() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(var(--c) 30% 40%)}\n"),
            "a {\
         \n  b: hwb(var(--c) 30% 40%);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // wrong result
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.hwb($channels: 180 30% 40% / 0.4)}\n"),
        "a {\
         \n  b: hsla(180, 33.3333333333%, 45%, 0.4);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn no_alpha() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(180 30% 40%)}\n"),
        "a {\
         \n  b: hsl(180, 33.3333333333%, 45%);\
         \n}\n"
    );
}
mod relative_color {
    #[allow(unused)]
    use super::runner;

    mod calc {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn alpha() {
            assert_eq!(
                runner()
                    .ok("a {b: hwb(from #aaa calc(h + 180deg) w b / 25%)}\n"),
                "a {\
         \n  b: hwb(from #aaa calc(h + 180deg) w b/25%);\
         \n}\n"
            );
        }
        #[test]
        fn no_alpha() {
            assert_eq!(
                runner().ok("a {b: hwb(from #aaa calc(h + 180deg) w b)}\n"),
                "a {\
         \n  b: hwb(from #aaa calc(h + 180deg) w b);\
         \n}\n"
            );
        }
    }
    mod different_case {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn alpha() {
            assert_eq!(
                runner().ok("a {b: hwb(From #aaa h w b / 25%)}\n"),
                "a {\
         \n  b: hwb(From #aaa h w b/25%);\
         \n}\n"
            );
        }
        #[test]
        fn no_alpha() {
            assert_eq!(
                runner().ok("a {b: hwb(From #aaa h w b)}\n"),
                "a {\
         \n  b: hwb(From #aaa h w b);\
         \n}\n"
            );
        }
    }
    mod error {
        #[allow(unused)]
        use super::runner;

        mod quoted {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // missing error
            fn alpha() {
                assert_eq!(
        runner().err(
            "a {b: hwb(\"from\" #aaa h w b / 25%)}\n"
        ),
        "Error: $channels: Expected hue channel to be a number, was \"from\".\
         \n  ,\
         \n1 | a {b: hwb(\"from\" #aaa h w b / 25%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
            }
            #[test]
            #[ignore] // missing error
            fn no_alpha() {
                assert_eq!(
        runner().err(
            "a {b: hwb(\"from\" #aaa h w b)}\n"
        ),
        "Error: $channels: Expected hue channel to be a number, was \"from\".\
         \n  ,\
         \n1 | a {b: hwb(\"from\" #aaa h w b)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
            }
        }
        mod wrong_keyword {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // missing error
            fn alpha() {
                assert_eq!(
        runner().err(
            "a {b: hwb(c #aaa h w b / 25%)}\n"
        ),
        "Error: $channels: Expected hue channel to be a number, was c.\
         \n  ,\
         \n1 | a {b: hwb(c #aaa h w b / 25%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
            }
            #[test]
            #[ignore] // missing error
            fn no_alpha() {
                assert_eq!(
        runner().err(
            "a {b: hwb(c #aaa h w b)}\n"
        ),
        "Error: $channels: Expected hue channel to be a number, was c.\
         \n  ,\
         \n1 | a {b: hwb(c #aaa h w b)}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
            }
        }
    }
    #[test]
    fn slash_list_alpha() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: hwb(list.slash(from #aaa h w b, 25%))}\n"),
            "a {\
         \n  b: hwb(from #aaa h w b / 25%);\
         \n}\n"
        );
    }
    mod test_static {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn alpha() {
            assert_eq!(
                runner().ok("a {b: hwb(from #aaa h w b / 25%)}\n"),
                "a {\
         \n  b: hwb(from #aaa h w b/25%);\
         \n}\n"
            );
        }
        #[test]
        fn no_alpha() {
            assert_eq!(
                runner().ok("a {b: hwb(from #aaa h w b)}\n"),
                "a {\
         \n  b: hwb(from #aaa h w b);\
         \n}\n"
            );
        }
    }
    mod var {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn alpha() {
            assert_eq!(
                runner().ok("a {b: hwb(from var(--c) h w b / 25%)}\n"),
                "a {\
         \n  b: hwb(from var(--c) h w b/25%);\
         \n}\n"
            );
        }
        #[test]
        fn no_alpha() {
            assert_eq!(
                runner().ok("a {b: hwb(from var(--c) h w b)}"),
                "a {\
         \n  b: hwb(from var(--c) h w b);\
         \n}\n"
            );
        }
    }
}
mod whiteness {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0 101% 40%)}\n"),
            "a {\
         \n  b: hsl(0, 0%, 71.6312056738%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0 -1% 40%)}\n"),
            "a {\
         \n  b: hsl(0, 103.3898305085%, 29.5%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn var() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0 var(--c) 40%)}\n"),
            "a {\
         \n  b: hwb(0 var(--c) 40%);\
         \n}\n"
        );
    }
}
