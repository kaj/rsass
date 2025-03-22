//! Tests auto-converted from "sass-spec/spec/directives/if/comment.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("comment")
}

mod test_else {
    use super::runner;

    mod before_block {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn loud() {
            assert_eq!(
                runner().ok("@if true {}\
             \n@else /**/ {}\n"),
                ""
            );
        }
        #[test]
        fn silent() {
            assert_eq!(
                runner().ok("@if true {}\
             \n@else //\
             \n  {}\n"),
                ""
            );
        }
    }
}
mod else_if {
    use super::runner;

    mod after_condition {
        use super::runner;

        #[test]
        fn loud() {
            assert_eq!(
                runner().ok("@if true {}\
             \n@else if true /**/ {}\n"),
                ""
            );
        }
        #[test]
        fn silent() {
            assert_eq!(
                runner().ok("@if true {}\
             \n@else if true //\
             \n  {}\n"),
                ""
            );
        }
    }
    mod before_condition {
        use super::runner;

        #[test]
        fn loud() {
            assert_eq!(
                runner().ok("@if true {}\
             \n@else if /**/ true {}\n"),
                ""
            );
        }
        #[test]
        fn silent() {
            assert_eq!(
                runner().ok("@if true {}\
             \n@else if //\
             \n  true {}\n"),
                ""
            );
        }
    }
    mod before_if {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn loud() {
            assert_eq!(
                runner().ok("@if true {}\
             \n@else /**/ if true {}\n"),
                ""
            );
        }
        #[test]
        fn silent() {
            assert_eq!(
                runner().ok("@if true {}\
             \n@else //\
             \n  if true {}\n"),
                ""
            );
        }
    }
}
mod test_if {
    use super::runner;

    mod after_condition {
        use super::runner;

        #[test]
        fn loud() {
            assert_eq!(runner().ok("@if true /**/ {}\n"), "");
        }
        #[test]
        fn silent() {
            assert_eq!(
                runner().ok("@if true //\
             \n  {}\n"),
                ""
            );
        }
    }
    mod before_condition {
        use super::runner;

        #[test]
        fn loud() {
            assert_eq!(runner().ok("@if /**/ true {}\n"), "");
        }
        #[test]
        fn silent() {
            assert_eq!(
                runner().ok("@if //\
             \n  true {}\n"),
                ""
            );
        }
    }
}
