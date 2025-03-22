//! Tests auto-converted from "sass-spec/spec/directives/mixin/comment.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("comment")
}

mod content {
    use super::runner;

    mod after_args {
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
    use super::runner;

    mod after_args {
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
    use super::runner;

    mod after_args {
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
