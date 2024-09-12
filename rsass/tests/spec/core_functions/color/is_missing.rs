//! Tests auto-converted from "sass-spec/spec/core_functions/color/is_missing.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("is_missing")
}

mod error {
    #[allow(unused)]
    use super::runner;

    mod channel {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn unquoted() {
            assert_eq!(
                runner().err(
                    "@use \"sass:color\";\
             \na {b: color.is-missing(hsl(0deg 50% 50%), hue)}\n"
                ),
                "Error: $channel: Expected hue to be a quoted string.\
         \n  ,\
         \n2 | a {b: color.is-missing(hsl(0deg 50% 50%), hue)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn wrong_case() {
            assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.is-missing(black, \"RED\")}\n"
        ),
        "Error: $channel: Color black doesn\'t have a channel named \"RED\".\
         \n  ,\
         \n2 | a {b: color.is-missing(black, \"RED\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
        #[test]
        #[ignore] // wrong error
        fn wrong_space() {
            assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.is-missing(black, \"hue\")}\n"
        ),
        "Error: $channel: Color black doesn\'t have a channel named \"hue\".\
         \n  ,\
         \n2 | a {b: color.is-missing(black, \"hue\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
    }
    #[test]
    #[ignore] // wrong error
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.is-missing(black)}\n"
            ),
            "Error: Missing argument $channel.\
         \n  ,--> input.scss\
         \n2 | a {b: color.is-missing(black)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function is-missing($color, $channel) {\
         \n  |           ============================ declaration\
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
             \na {b: color.is-missing(black, \"red\", rgb)}\n"
            ),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: color.is-missing(black, \"red\", rgb)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function is-missing($color, $channel) {\
         \n  |           ============================ declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    mod test_type {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn channel() {
            assert_eq!(
                runner().err(
                    "@use \"sass:color\";\
             \na {b: color.is-missing(black, red)}\n"
                ),
                "Error: $channel: red is not a string.\
         \n  ,\
         \n2 | a {b: color.is-missing(black, red)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn color() {
            assert_eq!(
                runner().err(
                    "@use \"sass:color\";\
             \na {b: color.is-missing(\"black\", \"red\")}\n"
                ),
                "Error: $color: \"black\" is not a color.\
         \n  ,\
         \n2 | a {b: color.is-missing(\"black\", \"red\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
}
mod test_false {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn explicit() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.is-missing(#abcdef, \"red\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    mod not_powerless {
        #[allow(unused)]
        use super::runner;

        mod converted {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn legacy() {
                assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.is-missing(color.to-space(#aaaaab, hsl), \"hue\")}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
            }
            #[test]
            #[ignore] // unexepected error
            fn non_legacy() {
                assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.is-missing(color.to-space(#aaaaab, lch), \"hue\")}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
            }
        }
        mod direct {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn legacy() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \na {b: color.is-missing(hsl(0deg 50% 1%), \"hue\")}\n"),
                    "a {\
         \n  b: false;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn non_legacy() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \na {b: color.is-missing(lch(50% 1% 0deg), \"hue\")}\n"),
                    "a {\
         \n  b: false;\
         \n}\n"
                );
            }
        }
    }
    mod powerless {
        #[allow(unused)]
        use super::runner;

        mod converted {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn legacy() {
                assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.is-missing(color.to-space(#aaaaaa, hsl), \"hue\")}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
            }
        }
        mod direct {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn legacy() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \na {b: color.is-missing(hsl(0deg 50% 0%), \"hue\")}\n"),
                    "a {\
         \n  b: false;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn non_legacy() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \na {b: color.is-missing(lch(50% 0% 0deg), \"hue\")}\n"),
                    "a {\
         \n  b: false;\
         \n}\n"
                );
            }
        }
    }
}
#[test]
#[ignore] // unexepected error
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.is-missing($color: black, $channel: \"red\")}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
mod test_true {
    #[allow(unused)]
    use super::runner;

    mod explicit {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn legacy() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.is-missing(rgb(none 30 100), \"red\")}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn non_legacy() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.is-missing(lab(50% 30 none), \"b\")}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
    }
    mod powerless {
        #[allow(unused)]
        use super::runner;

        mod converted {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn non_legacy() {
                assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.is-missing(color.to-space(#aaaaaa, lch), \"hue\")}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
            }
        }
    }
}
