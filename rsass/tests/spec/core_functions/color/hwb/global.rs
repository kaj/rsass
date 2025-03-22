//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/global.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("global")
}

mod alpha {
    use super::runner;

    mod clamped {
        use super::runner;

        #[test]
        fn above() {
            assert_eq!(
                runner().ok("a {b: hwb(0 30% 40% / 1.1)}\n"),
                "a {\
         \n  b: hsl(0, 33.3333333333%, 45%);\
         \n}\n"
            );
        }
        #[test]
        fn below() {
            assert_eq!(
                runner().ok("a {b: hwb(0 30% 40% / -0.1)}\n"),
                "a {\
         \n  b: hsla(0, 33.3333333333%, 45%, 0);\
         \n}\n"
            );
        }
    }
    mod in_gamut {
        use super::runner;

        #[test]
        fn named() {
            assert_eq!(
                runner().ok("a {b: hwb($channels: 180 30% 40% / 0.4)}\n"),
                "a {\
         \n  b: hsla(180, 33.3333333333%, 45%, 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn opaque() {
            assert_eq!(
                runner().ok("a {b: hwb(180 30% 40% / 1)}\n"),
                "a {\
         \n  b: hsl(180, 33.3333333333%, 45%);\
         \n}\n"
            );
        }
        #[test]
        fn parenthesized() {
            assert_eq!(
        runner().ok(
            "// Extra parens shouldn\'t cause the slash to be forced into division.\
             \na {b: (hwb(180 30% 40% / 0.4))}\n"
        ),
        "a {\
         \n  b: hsla(180, 33.3333333333%, 45%, 0.4);\
         \n}\n"
    );
        }
        #[test]
        fn partial() {
            assert_eq!(
                runner().ok("a {b: hwb(180 30% 40% / 0.5)}\n"),
                "a {\
         \n  b: hsla(180, 33.3333333333%, 45%, 0.5);\
         \n}\n"
            );
        }
        #[test]
        fn transparent() {
            assert_eq!(
                runner().ok("a {b: hwb(180 30% 40% / 0)}\n"),
                "a {\
         \n  b: hsla(180, 33.3333333333%, 45%, 0);\
         \n}\n"
            );
        }
    }
    mod missing {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn slash() {
            assert_eq!(
                runner().ok("a {b: hwb(0 30% 40% / none)}\n"),
                "a {\
         \n  b: hwb(0deg 30% 40% / none);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn slash_list() {
            assert_eq!(
                runner().ok("@use \'sass:list\';\
             \na {b: hwb(list.slash(0 30% 40%, none))}\n"),
                "a {\
         \n  b: hwb(0deg 30% 40% / none);\
         \n}\n"
            );
        }
    }
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blackness() {
        assert_eq!(
            runner().ok("a {b: hwb(0 30% none)}\n"),
            "a {\
         \n  b: hwb(0deg 30% none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("a {b: hwb(none 30% 40%)}\n"),
            "a {\
         \n  b: hwb(none 30% 40%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn whiteness() {
        assert_eq!(
            runner().ok("a {b: hwb(0 none 40%)}\n"),
            "a {\
         \n  b: hwb(0deg none 40%);\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("a {b: hwb($channels: 180 30% 40% / 0.4)}\n"),
        "a {\
         \n  b: hsla(180, 33.3333333333%, 45%, 0.4);\
         \n}\n"
    );
}
#[test]
fn no_alpha() {
    assert_eq!(
        runner().ok("a {b: hwb(180 30% 40%)}\n"),
        "a {\
         \n  b: hsl(180, 33.3333333333%, 45%);\
         \n}\n"
    );
}
mod relative_color {
    use super::runner;

    mod calc {
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
        use super::runner;

        mod quoted {
            use super::runner;

            #[test]
            #[ignore] // wrong error
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
            #[ignore] // wrong error
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
            use super::runner;

            #[test]
            #[ignore] // wrong error
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
            #[ignore] // wrong error
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
