//! Tests auto-converted from "sass-spec/spec/css/functions/url.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("url")
}

mod exclam {
    use super::runner;

    #[test]
    fn middle() {
        assert_eq!(
            runner().ok("a {b: url(http://c.d/e!f)}\n"),
            "a {\
         \n  b: url(http://c.d/e!f);\
         \n}\n"
        );
    }
    #[test]
    fn only() {
        assert_eq!(
            runner().ok("a {b: url(!)}\n"),
            "a {\
         \n  b: url(!);\
         \n}\n"
        );
    }
}
