//! Tests auto-converted from "sass-spec/spec/css/moz_document/comment.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("comment")
}

mod after_arg {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn loud() {
        assert_eq!(
            runner().ok("@-moz-document url-prefix(a) /**/ {}\n"),
            "@-moz-document url-prefix(a) {}\n"
        );
    }
    #[test]
    fn silent() {
        assert_eq!(
            runner().ok("@-moz-document url-prefix(a) //\
             \n  {}\n"),
            "@-moz-document url-prefix(a) {}\n"
        );
    }
}
mod before_arg {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn loud() {
        assert_eq!(
            runner().ok("@-moz-document /**/ url-prefix(a) {}\n"),
            "@-moz-document url-prefix(a) {}\n"
        );
    }
    #[test]
    fn silent() {
        assert_eq!(
            runner().ok("@-moz-document //\
             \n  url-prefix(a) {}\n"),
            "@-moz-document url-prefix(a) {}\n"
        );
    }
}
