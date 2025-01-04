//! Tests auto-converted from "sass-spec/spec/callable/parameters.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("parameters")
}

mod function {
    #[allow(unused)]
    use super::runner;

    mod error {
        #[allow(unused)]
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
    }
    mod trailing_comma {
        #[allow(unused)]
        use super::runner;

        mod default {
            #[allow(unused)]
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
            #[allow(unused)]
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
    #[allow(unused)]
    use super::runner;

    mod error {
        #[allow(unused)]
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
    }
    mod trailing_comma {
        #[allow(unused)]
        use super::runner;

        mod default {
            #[allow(unused)]
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
            #[allow(unused)]
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
