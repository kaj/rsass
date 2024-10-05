//! Tests auto-converted from "sass-spec/spec/directives/mixin.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mixin")
}

mod comment {
    #[allow(unused)]
    use super::runner;

    mod content {
        #[allow(unused)]
        use super::runner;

        mod after_args {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(runner().ok("@mixin a {@content() /**/}\n"), "");
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("@mixin a {\
             \n  @content() //\
             \n}\n"),
                    ""
                );
            }
        }
        mod after_content {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(runner().ok("@mixin a {@content /**/}\n"), "");
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("@mixin a {\
             \n  @content //\
             \n}\n"),
                    ""
                );
            }
        }
    }
    mod include {
        #[allow(unused)]
        use super::runner;

        mod after_args {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(
                    runner().ok("@mixin a {}\
             \n@include a() /**/\n"),
                    ""
                );
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("@mixin a {}\
             \n@include a() //\n"),
                    ""
                );
            }
        }
        mod after_name {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(
                    runner().ok("@mixin a {}\
             \n@include a /**/\n"),
                    ""
                );
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("@mixin a {}\
             \n@include a //\n"),
                    ""
                );
            }
        }
        mod after_using {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(
                    runner().ok("@mixin a {@content}\
             \n@include a() using /**/ () {}\n"),
                    ""
                );
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("@mixin a {@content}\
             \n@include a() using //\
             \n  () {}\n"),
                    ""
                );
            }
        }
        mod after_using_arglist {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(
                    runner().ok("@mixin a {@content}\
             \n@include a() using () /**/ {}\n"),
                    ""
                );
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("@mixin a {@content}\
             \n@include a() using () //\
             \n  {}\n"),
                    ""
                );
            }
        }
        mod before_block {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(
                    runner().ok("@mixin a {@content}\
             \n@include a() /**/ {}\n"),
                    ""
                );
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("@mixin a {@content}\
             \n@include a() //\
             \n  {}\n"),
                    ""
                );
            }
        }
        mod before_name {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(
                    runner().ok("@mixin a {}\
             \n@include /**/ a\n"),
                    ""
                );
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("@mixin a {}\
             \n@include //\
             \n  a\n"),
                    ""
                );
            }
        }
        mod before_using {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(
                    runner().ok("@mixin a {@content}\
             \n@include a() /**/ using () {}\n"),
                    ""
                );
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("@mixin a {@content}\
             \n@include a() //\
             \n  using () {}\n"),
                    ""
                );
            }
        }
    }
    mod mixin {
        #[allow(unused)]
        use super::runner;

        mod after_args {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(runner().ok("@mixin a() /**/ {}\n"), "");
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("@mixin a() //\
             \n  {}\n"),
                    ""
                );
            }
        }
        mod before_name {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(runner().ok("@mixin /**/ a {}\n"), "");
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("@mixin //\
             \n  a {}\n"),
                    ""
                );
            }
        }
    }
}
#[test]
fn custom_ident_include() {
    assert_eq!(
        runner().ok("@mixin __a() {b: c}\
             \nd {@include --a}\n"),
        "d {\
         \n  b: c;\
         \n}\n"
    );
}
#[test]
fn custom_ident_name() {
    assert_eq!(
        runner().ok("@mixin --a {b: c}\
             \nd {@include --a}\n"),
        "d {\
         \n  b: c;\
         \n}\n"
    );
}
#[test]
fn double_underscore_name() {
    assert_eq!(
        runner().ok("@mixin __a() {b: c}\
             \nd {@include __a}\n"),
        "d {\
         \n  b: c;\
         \n}\n"
    );
}
