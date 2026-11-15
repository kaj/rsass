//! Tests auto-converted from "sass-spec/spec/callable/parameters.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("parameters")
}

mod function {
    use super::runner;

    mod error {
        use super::runner;

        #[test]
        fn comma_only() {
            assert_eq!(
                runner().err("@function a( , ) {}\n"),
                "Error: expected \")\".\
         \n  ,\
         \n1 | @function a( , ) {}\
         \n  |              ^\
         \n  \'\
         \n  input.scss 1:14  root stylesheet",
            );
        }
        mod splat {
            use super::runner;

            #[test]
            fn before_final() {
                assert_eq!(
                    runner().err("@function a($b..., $c) {@return null}\n"),
                    "Error: expected \")\".\
         \n  ,\
         \n1 | @function a($b..., $c) {@return null}\
         \n  |                    ^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
                );
            }
        }
    }
    mod trailing_comma {
        use super::runner;

        mod default {
            use super::runner;

            #[test]
            fn after_no_default() {
                assert_eq!(
                    runner().ok("@function a($b, $c: 1, ) {\
             \n  @return $c;\
             \n}\n\
             \nd {e: a(2)}\n"),
                    "d {\
         \n  e: 1;\
         \n}\n"
                );
            }
            #[test]
            fn alone() {
                assert_eq!(
                    runner().ok("@function a($b: 1, ) {\
             \n  @return $b;\
             \n}\n\
             \nc {d: a()}\n"),
                    "c {\
         \n  d: 1;\
         \n}\n"
                );
            }
        }
        #[test]
        fn no_default() {
            assert_eq!(
                runner().ok("@function a($b, ) {\
             \n  @return $b;\
             \n}\n\
             \nc {d: a(e)}\n"),
                "c {\
         \n  d: e;\
         \n}\n"
            );
        }
        mod rest {
            use super::runner;

            #[test]
            fn after_both() {
                assert_eq!(
                    runner().ok("@function a($b, $c: 1, $d..., ) {\
             \n  @return $d;\
             \n}\n\
             \ne {f: a(2, 3, 4)}\n"),
                    "e {\
         \n  f: 4;\
         \n}\n"
                );
            }
            #[test]
            fn after_default() {
                assert_eq!(
                    runner().ok("@function a($b: 1, $c..., ) {\
             \n  @return $c;\
             \n}\n\
             \nd {e: a(2, 3)}\n"),
                    "d {\
         \n  e: 3;\
         \n}\n"
                );
            }
            #[test]
            fn after_no_default() {
                assert_eq!(
                    runner().ok("@function a($b, $c..., ) {\
             \n  @return $c;\
             \n}\n\
             \nd {e: a(1, 2)}\n"),
                    "d {\
         \n  e: 2;\
         \n}\n"
                );
            }
            #[test]
            fn alone() {
                assert_eq!(
                    runner().ok("@function a($b..., ) {\
             \n  @return $b;\
             \n}\n\
             \nc {d: a(1)}\n"),
                    "c {\
         \n  d: 1;\
         \n}\n"
                );
            }
        }
    }
}
mod mixin {
    use super::runner;

    mod error {
        use super::runner;

        #[test]
        fn comma_only() {
            assert_eq!(
                runner().err("@mixin a( , ) {}\n"),
                "Error: expected \")\".\
         \n  ,\
         \n1 | @mixin a( , ) {}\
         \n  |           ^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
            );
        }
        mod splat {
            use super::runner;

            #[test]
            fn before_final() {
                assert_eq!(
                    runner().err("@mixin a($b..., $c) {}\n"),
                    "Error: expected \")\".\
         \n  ,\
         \n1 | @mixin a($b..., $c) {}\
         \n  |                 ^\
         \n  \'\
         \n  input.scss 1:17  root stylesheet",
                );
            }
        }
    }
    mod trailing_comma {
        use super::runner;

        mod default {
            use super::runner;

            #[test]
            fn after_no_default() {
                assert_eq!(
                    runner().ok("@mixin a($b, $c: 1, ) {\
             \n  d: $c;\
             \n}\n\
             \ne {@include a(2)}\n"),
                    "e {\
         \n  d: 1;\
         \n}\n"
                );
            }
            #[test]
            fn alone() {
                assert_eq!(
                    runner().ok("@mixin a($b: 1, ) {\
             \n  c: $b;\
             \n}\n\
             \nd {@include a}\n"),
                    "d {\
         \n  c: 1;\
         \n}\n"
                );
            }
        }
        #[test]
        fn no_default() {
            assert_eq!(
                runner().ok("@mixin a($b, ) {\
             \n  c: $b;\
             \n}\n\
             \nd {@include a(1)}\n"),
                "d {\
         \n  c: 1;\
         \n}\n"
            );
        }
        mod rest {
            use super::runner;

            #[test]
            fn after_both() {
                assert_eq!(
                    runner().ok("@mixin a($b, $c: 1, $d..., ) {\
             \n  d: $d;\
             \n}\n\
             \ne {@include a(2, 3, 4)}\n"),
                    "e {\
         \n  d: 4;\
         \n}\n"
                );
            }
            #[test]
            fn after_default() {
                assert_eq!(
                    runner().ok("@mixin a($b: 1, $c..., ) {\
             \n  d: $c;\
             \n}\n\
             \ne {@include a(2, 3)}\n"),
                    "e {\
         \n  d: 3;\
         \n}\n"
                );
            }
            #[test]
            fn after_no_default() {
                assert_eq!(
                    runner().ok("@mixin a($b, $c..., ) {\
             \n  d: $c;\
             \n}\n\
             \ne {@include a(1, 2)}\n"),
                    "e {\
         \n  d: 2;\
         \n}\n"
                );
            }
            #[test]
            fn alone() {
                assert_eq!(
                    runner().ok("@mixin a($b..., ) {\
             \n  c: $b;\
             \n}\n\
             \nd {@include a(1)}\n"),
                    "d {\
         \n  c: 1;\
         \n}\n"
                );
            }
        }
    }
}
mod rest {
    use super::runner;

    mod named {
        use super::runner;

        mod escape {
            use super::runner;

            mod literal {
                use super::runner;

                #[test]
                #[ignore] // unexepected error
                fn non_normalized() {
                    assert_eq!(
                        runner().ok("@use \'sass:meta\';\
             \n@mixin a($args...) {b: meta.inspect(meta.keywords($args))}\
             \nc {@include a($\\24: d)}\n"),
                        "c {\
         \n  b: (\\$: d);\
         \n}\n"
                    );
                }
                #[test]
                #[ignore] // unexepected error
                fn normalized() {
                    assert_eq!(
                        runner().ok("@use \'sass:meta\';\
             \n@mixin a($args...) {b: meta.inspect(meta.keywords($args))}\
             \nc {@include a($\\$: d)}\n"),
                        "c {\
         \n  b: (\\$: d);\
         \n}\n"
                    );
                }
            }
            mod rest {
                use super::runner;

                mod quoted {
                    use super::runner;

                    #[test]
                    #[ignore] // wrong result
                    fn dash_insensitive() {
                        assert_eq!(
                            runner().ok("@use \'sass:meta\';\
             \n@mixin a($args...) {b: meta.inspect(meta.keywords($args))}\
             \nc {@include a((\"d\\\\5f e\": f)...)}\n"),
                            "c {\
         \n  b: (d\\5f e: f);\
         \n}\n"
                        );
                    }
                    #[test]
                    #[ignore] // wrong result
                    fn non_normalized() {
                        assert_eq!(
                            runner().ok("@use \'sass:meta\';\
             \n@mixin a($args...) {b: meta.inspect(meta.keywords($args))}\
             \nc {@include a((\"\\\\24\": d)...)}\n"),
                            "c {\
         \n  b: (\\24: d);\
         \n}\n"
                        );
                    }
                    #[test]
                    #[ignore] // wrong result
                    fn normalized() {
                        assert_eq!(
                            runner().ok("@use \'sass:meta\';\
             \n@mixin a($args...) {b: meta.inspect(meta.keywords($args))}\
             \nc {@include a((\"\\\\$\": d)...)}\n"),
                            "c {\
         \n  b: (\\$: d);\
         \n}\n"
                        );
                    }
                    #[test]
                    fn semantic() {
                        assert_eq!(
                            runner().ok("@use \'sass:meta\';\
             \n@mixin a($args...) {b: meta.inspect(meta.keywords($args))}\
             \nc {@include a((\"$\": d)...)}\n"),
                            "c {\
         \n  b: ($: d);\
         \n}\n"
                        );
                    }
                }
                mod unquoted {
                    use super::runner;

                    #[test]
                    fn dash_insensitive() {
                        assert_eq!(
                            runner().ok("@use \'sass:meta\';\
             \n@use \'sass:string\';\
             \n@mixin a($args...) {b: meta.inspect(meta.keywords($args))}\
             \nc {@include a((string.unquote(\"d\\\\5f e\"): f)...)}\n"),
                            "c {\
         \n  b: (d\\5f e: f);\
         \n}\n"
                        );
                    }
                    #[test]
                    fn non_normalized() {
                        assert_eq!(
                            runner().ok("@use \'sass:meta\';\
             \n@use \'sass:string\';\
             \n@mixin a($args...) {b: meta.inspect(meta.keywords($args))}\
             \nc {@include a((string.unquote(\"\\\\24\"): d)...)}\n"),
                            "c {\
         \n  b: (\\24: d);\
         \n}\n"
                        );
                    }
                    #[test]
                    fn normalized() {
                        assert_eq!(
                            runner().ok("@use \'sass:meta\';\
             \n@mixin a($args...) {b: meta.inspect(meta.keywords($args))}\
             \nc {@include a((\\$: d)...)}\n"),
                            "c {\
         \n  b: (\\$: d);\
         \n}\n"
                        );
                    }
                    #[test]
                    fn semantic() {
                        assert_eq!(
                            runner().ok("@use \'sass:meta\';\
             \n@use \'sass:string\';\
             \n@mixin a($args...) {b: meta.inspect(meta.keywords($args))}\
             \nc {@include a((string.unquote(\"$\"): d)...)}\n"),
                            "c {\
         \n  b: ($: d);\
         \n}\n"
                        );
                    }
                }
            }
        }
    }
}
