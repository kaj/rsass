//! Tests auto-converted from "sass-spec/spec/css/moz_document/whitespace.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("whitespace")
}

mod after_arg {
    use super::runner;

    #[test]
    fn scss() {
        assert_eq!(
            runner().ok("@-moz-document url-prefix(a)\
             \n  {}\n"),
            "@-moz-document url-prefix(a) {}\n"
        );
    }
}
mod before_arg {
    use super::runner;

    #[test]
    fn scss() {
        assert_eq!(
            runner().ok("@-moz-document\
             \n  url-prefix(a) {}\n"),
            "@-moz-document url-prefix(a) {}\n"
        );
    }
}
