//! Tests auto-converted from "sass-spec/spec/callable/whitespace.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("whitespace")
}

mod newlines {
    use super::runner;

    mod function {
        use super::runner;

        mod after_arg {
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@function a($b\
             \n  , $c){}\n"),
                    ""
                );
            }
        }
        mod after_colon {
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@function a($b, $c:\
             \n    d){}\n"),
                    ""
                );
            }
        }
        mod after_comma {
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@function a($b,\
             \n  $c){}\n"),
                    ""
                );
            }
        }
        mod after_list {
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@function a($b...\
             \n  ){}\n"),
                    ""
                );
            }
        }
        mod after_paren {
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@function a(\
             \n  $b, $c){}\n"),
                    ""
                );
            }
        }
        mod before_colon {
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@function a($b, $c\
             \n    :d){}\n"),
                    ""
                );
            }
        }
        mod before_list {
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@function a($b\
             \n  ...){}\n"),
                    ""
                );
            }
        }
        mod before_list_arg {
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@function a(\
             \n  $b...){}\n"),
                    ""
                );
            }
        }
        mod before_paren {
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@function a($b, $c\
             \n  ){}\n"),
                    ""
                );
            }
        }
    }
    mod function_invocation {
        use super::runner;

        mod after_arg {
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@function a($b, $c){@return null}\
             \n$d: a(e\
             \n  , f)\n"),
                    ""
                );
            }
        }
        mod after_colon {
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@function a($b, $c:d){@return null}\
             \n$e: a(f, $c:\
             \n  g)\n"),
                    ""
                );
            }
        }
        mod after_comma {
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@function a($b, $c){@return null}\
             \n$d: a(e,\
             \n  f)\n"),
                    ""
                );
            }
        }
        mod after_list {
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@function a($b, $c){@return null}\
             \n$d: e f;\
             \n$g: a($d...\
             \n  )\n"),
                    ""
                );
            }
        }
        mod after_paren {
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@function a($b, $c){@return null}\
             \n$d: a(\
             \n  e, f)"),
                    ""
                );
            }
        }
        mod before_colon {
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@function a($b, $c:d){@return null}\
             \n$e: a(f, $c\
             \n  :g)\n"),
                    ""
                );
            }
        }
        mod before_list {
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@function a($b, $c){@return null}\
             \n$d: e f;\
             \n$g: a($d\
             \n  ...)\n"),
                    ""
                );
            }
        }
        mod before_list_arg {
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@function a($b, $c){@return null}\
             \n$d: e f;\
             \n$g: a(\
             \n  $d...)\n"),
                    ""
                );
            }
        }
        mod before_paren {
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@function a($b, $c:d){@return null}\
             \n$e: a(f, $c: g\
             \n  )\n"),
                    ""
                );
            }
        }
    }
    mod mixin {
        use super::runner;

        mod after_arg {
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@mixin a($b\
             \n  , $c){}\n"),
                    ""
                );
            }
        }
        mod after_colon {
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@mixin a($b, $c:\
             \n    d){}\n"),
                    ""
                );
            }
        }
        mod after_comma {
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@mixin a($b,\
             \n  $c){}\n"),
                    ""
                );
            }
        }
        mod after_list {
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@mixin a($b...\
             \n  ){}\n"),
                    ""
                );
            }
        }
        mod after_paren {
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@mixin a(\
             \n  $b, $c){}\n"),
                    ""
                );
            }
        }
        mod before_colon {
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@mixin a($b, $c\
             \n    :d){}\n"),
                    ""
                );
            }
        }
        mod before_list {
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@mixin a($b\
             \n  ...){}\n"),
                    ""
                );
            }
        }
        mod before_list_arg {
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@mixin a(\
             \n  $b...){}\n"),
                    ""
                );
            }
        }
        mod before_paren {
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@mixin a($b, $c\
             \n  ){}\n"),
                    ""
                );
            }
        }
    }
}
