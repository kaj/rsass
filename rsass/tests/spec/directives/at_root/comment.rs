//! Tests auto-converted from "sass-spec/spec/directives/at_root/comment.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("comment")
}

mod after_colon {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn loud() {
        assert_eq!(runner().ok("@at-root (without: /**/ media) {}\n"), "");
    }
    #[test]
    #[ignore] // unexepected error
    fn silent() {
        assert_eq!(
            runner().ok("@at-root (without: //\
             \n  media) {}\n"),
            ""
        );
    }
}
mod after_open_paren {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn loud() {
        assert_eq!(runner().ok("@at-root (/**/ without: media) {}\n"), "");
    }
    #[test]
    #[ignore] // unexepected error
    fn silent() {
        assert_eq!(
            runner().ok("@at-root (//\
             \n  without: media) {}\n"),
            ""
        );
    }
}
mod after_query {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn loud() {
        assert_eq!(runner().ok("@at-root (without: media) /**/ {}\n"), "");
    }
    #[test]
    #[ignore] // unexepected error
    fn silent() {
        assert_eq!(
            runner().ok("@at-root (without: media) //\
             \n  {}\n"),
            ""
        );
    }
}
mod before_close_paren {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn loud() {
        assert_eq!(runner().ok("@at-root (without: media /**/) {}\n"), "");
    }
    #[test]
    #[ignore] // unexepected error
    fn silent() {
        assert_eq!(
            runner().ok("@at-root (without: media //\
             \n  ) {}\n"),
            ""
        );
    }
}
mod before_colon {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn loud() {
        assert_eq!(runner().ok("@at-root (without /**/ : media) {}\n"), "");
    }
    #[test]
    #[ignore] // unexepected error
    fn silent() {
        assert_eq!(
            runner().ok("@at-root (without //\
             \n  : media) {}\n"),
            ""
        );
    }
}
mod before_query {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn loud() {
        assert_eq!(runner().ok("@at-root /**/ (without: media) {}\n"), "");
    }
    #[test]
    #[ignore] // unexepected error
    fn silent() {
        assert_eq!(
            runner().ok("@at-root //\
             \n  (without: media) {}\n"),
            ""
        );
    }
}
mod no_query {
    use super::runner;

    #[test]
    fn loud() {
        assert_eq!(runner().ok("@at-root /**/ {}\n"), "");
    }
    #[test]
    fn silent() {
        assert_eq!(
            runner().ok("@at-root //\
             \n  {}\n"),
            ""
        );
    }
}
