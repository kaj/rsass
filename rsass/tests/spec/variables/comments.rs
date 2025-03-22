//! Tests auto-converted from "sass-spec/spec/variables/comments.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("comments")
}

mod after_colon {
    use super::runner;

    #[test]
    fn loud() {
        assert_eq!(runner().ok("$a: /**/ b\n"), "");
    }
    #[test]
    fn silent() {
        assert_eq!(
            runner().ok("$a: //\
             \n  b\n"),
            ""
        );
    }
}
mod after_value {
    use super::runner;

    #[test]
    fn loud() {
        assert_eq!(runner().ok("$a: b /**/\n"), "");
    }
    #[test]
    fn silent() {
        assert_eq!(runner().ok("$a: b //\n"), "");
    }
}
mod before_colon {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn loud() {
        assert_eq!(runner().ok("$a /**/: b\n"), "");
    }
    #[test]
    fn silent() {
        assert_eq!(
            runner().ok("$a //\
             \n  : b\n"),
            ""
        );
    }
}
