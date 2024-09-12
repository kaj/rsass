//! Tests auto-converted from "sass-spec/spec/directives/if/comment.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("comment")
}

mod comment {
    #[allow(unused)]
    use super::runner;

    mod test_else {
        #[allow(unused)]
        use super::runner;

        mod before_block {
            #[allow(unused)]
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
        #[allow(unused)]
        use super::runner;

        mod after_condition {
            #[allow(unused)]
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
            #[allow(unused)]
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
            #[allow(unused)]
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
        #[allow(unused)]
        use super::runner;

        mod after_condition {
            #[allow(unused)]
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
            #[allow(unused)]
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
}
