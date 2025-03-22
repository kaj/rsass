//! Tests auto-converted from "sass-spec/spec/directives/at_root/whitespace.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("whitespace")
}

mod after_colon {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn scss() {
        assert_eq!(
            runner().ok("@at-root (without: \
             \n  media) {}\n"),
            ""
        );
    }
}
mod after_open_paren {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn scss() {
        assert_eq!(
            runner().ok("@at-root (\
             \n  without: media) {}\n"),
            ""
        );
    }
}
mod after_query {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn scss() {
        assert_eq!(
            runner().ok("@at-root (without: media) \
             \n  {}\n"),
            ""
        );
    }
}
mod before_close_paren {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn scss() {
        assert_eq!(
            runner().ok("@at-root (without: media\
             \n  ) {}\n"),
            ""
        );
    }
}
mod before_colon {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn scss() {
        assert_eq!(
            runner().ok("@at-root (without\
             \n : media) {}\n"),
            ""
        );
    }
}
mod before_query {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn scss() {
        assert_eq!(
            runner().ok("@at-root\
             \n  (without: media) {}\n"),
            ""
        );
    }
}
mod no_query {
    use super::runner;

    #[test]
    fn scss() {
        assert_eq!(
            runner().ok("@at-root\
             \n  {}\n"),
            ""
        );
    }
}
