//! Tests auto-converted from "sass-spec/spec/css/comment.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("comment")
}

mod converts_newlines {
    #[allow(unused)]
    use super::runner;

    mod sass {
        #[allow(unused)]
        use super::runner;
    }
    mod scss {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn cr() {
            assert_eq!(
                runner().ok("/* foo\r * bar */\n"),
                "/* foo\
         \n * bar */\n"
            );
        }
        #[test]
        fn ff() {
            assert_eq!(
                runner().ok("/* foo\u{c} * bar */\n"),
                "/* foo\
         \n * bar */\n"
            );
        }
    }
}
mod error {
    #[allow(unused)]
    use super::runner;

    mod loud {
        #[allow(unused)]
        use super::runner;

        mod interpolation {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn failure() {
                assert_eq!(
                    runner().err("/* #{$undefined} */\n"),
                    "Error: Undefined variable.\
         \n  ,\
         \n1 | /* #{$undefined} */\
         \n  |      ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:6  root stylesheet",
                );
            }
            #[test]
            #[ignore] // missing error
            fn unterminated() {
                assert_eq!(
                    runner().err("/* #{broken */\n"),
                    "Error: Expected expression.\
         \n  ,\
         \n1 | /* #{broken */\
         \n  |               ^\
         \n  \'\
         \n  input.scss 1:15  root stylesheet",
                );
            }
        }
        mod multi_line {
            #[allow(unused)]
            use super::runner;
        }
        mod unterminated {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // missing error
            fn scss() {
                assert_eq!(
                    runner().err(
                        "a {\
             \n  b: c /* d\
             \n}\n"
                    ),
                    "Error: expected more input.\
         \n  ,\
         \n3 | }\
         \n  |  ^\
         \n  \'\
         \n  input.scss 3:2  root stylesheet",
                );
            }
        }
    }
}
mod inline {
    #[allow(unused)]
    use super::runner;

    mod loud {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn scss() {
            assert_eq!(
                runner().ok("a {\
             \n  b: c /* d */ e;\
             \n}\n"),
                "a {\
         \n  b: c e;\
         \n}\n"
            );
        }
    }
    mod silent {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn scss() {
            assert_eq!(
                runner().ok("a {\
             \n  b: c // d\
             \n}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
    }
}
#[test]
fn multiple() {
    assert_eq!(
        runner().ok(".foo {\
             \n  /* Foo Bar */\
             \n  /* Baz Bang */ }\n"),
        ".foo {\
         \n  /* Foo Bar */\
         \n  /* Baz Bang */\
         \n}\n"
    );
}
#[test]
fn multiple_stars() {
    assert_eq!(
        runner().ok("a /***/ b {x: y}\
             \na /****/ b {x: y}\
             \na /* **/ b {x: y}\
             \na /** */ b {x: y}\n"),
        "a b {\
         \n  x: y;\
         \n}\
         \na b {\
         \n  x: y;\
         \n}\
         \na b {\
         \n  x: y;\
         \n}\
         \na b {\
         \n  x: y;\
         \n}\n"
    );
}
mod sourcemap {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn between_loads() {
        assert_eq!(
            runner().ok("@use \'sass:math\';\
             \n/*# sourceMappingURL=whatever */\
             \n@use \'sass:list\';\n\
             \na { b: c }\n"),
            "\
         \na {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    fn sourcemappingurl() {
        assert_eq!(
            runner().ok("a { b: c }\
             \n/*# sourceMappingURL=whatever */\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    fn sourceurl() {
        assert_eq!(
            runner().ok("a { b: c }\
             \n/*# sourceURL=whatever */\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
}
#[test]
fn weird_indentation() {
    assert_eq!(
        runner().ok(".foo {\
             \n    /* Foo\
             \n Bar\
             \nBaz */\
             \n  a: b; }\n"),
        ".foo {\
         \n  /* Foo\
         \n   Bar\
         \n  Baz */\
         \n  a: b;\
         \n}\n"
    );
}
