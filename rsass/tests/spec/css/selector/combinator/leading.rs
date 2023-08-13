//! Tests auto-converted from "sass-spec/spec/css/selector/combinator/leading.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("leading")
}

mod multiple {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn no_whitespace() {
        assert_eq!(runner().ok("> > a {b: c}\n"), "");
    }
    #[test]
    fn whitespace() {
        assert_eq!(runner().ok("+ ~ a {b: c}\n"), "");
    }
}
mod single {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn child() {
        assert_eq!(
            runner().ok("> a {b: c}\n"),
            "> a {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    fn next_sibling() {
        assert_eq!(
            runner().ok("+ a {b: c}\n"),
            "+ a {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    fn sibling() {
        assert_eq!(
            runner().ok("~ a {b: c}\n"),
            "~ a {\
         \n  b: c;\
         \n}\n"
        );
    }
}
