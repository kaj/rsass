//! Tests auto-converted from "sass-spec/spec/css/media/comment.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("comment")
}

mod after_query {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn loud() {
        assert_eq!(runner().ok("@media screen /**/ {}\n"), "");
    }
    #[test]
    fn silent() {
        assert_eq!(
            runner().ok("@media screen //\
             \n  {}\n"),
            ""
        );
    }
}
mod before_query {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn loud() {
        assert_eq!(runner().ok("@media /**/ screen {}\n"), "");
    }
    #[test]
    fn silent() {
        assert_eq!(
            runner().ok("@media //\
             \n  screen {}\n"),
            ""
        );
    }
}
