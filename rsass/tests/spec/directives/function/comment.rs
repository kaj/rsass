//! Tests auto-converted from "sass-spec/spec/directives/function/comment.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("comment")
}

mod function {
    use super::runner;

    mod after_args {
        use super::runner;

        #[test]
        fn loud() {
            assert_eq!(runner().ok("@function a() /**/ {}\n"), "");
        }
        #[test]
        fn silent() {
            assert_eq!(
                runner().ok("@function a() //\
             \n  {}\n"),
                ""
            );
        }
    }
    mod before_name {
        use super::runner;

        #[test]
        fn loud() {
            assert_eq!(runner().ok("@function /**/ a() {}\n"), "");
        }
        #[test]
        fn silent() {
            assert_eq!(
                runner().ok("@function //\
             \n  a() {}\n"),
                ""
            );
        }
    }
}
mod test_return {
    use super::runner;

    mod after_value {
        use super::runner;

        #[test]
        fn loud() {
            assert_eq!(runner().ok("@function a() {@return b /**/}\n"), "");
        }
        #[test]
        fn silent() {
            assert_eq!(
                runner().ok("@function a() {\
             \n  @return b //\
             \n}\n"),
                ""
            );
        }
    }
    mod before_value {
        use super::runner;

        #[test]
        fn loud() {
            assert_eq!(runner().ok("@function a() {@return /**/ b}\n"), "");
        }
        #[test]
        fn silent() {
            assert_eq!(
                runner().ok("@function a() {\
             \n  @return //\
             \n    b\
             \n}\n"),
                ""
            );
        }
    }
}
