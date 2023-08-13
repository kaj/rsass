//! Tests auto-converted from "sass-spec/spec/css/selector/combinator/middle.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("middle")
}

mod multiple {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn no_whitespace() {
        assert_eq!(runner().ok("a~>b {c: d}\n"), "");
    }
    #[test]
    fn whitespace() {
        assert_eq!(runner().ok("a > + b {c: d}\n"), "");
    }
}
mod single {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn child() {
        assert_eq!(
            runner().ok("a > b {c: d}\n"),
            "a > b {\
         \n  c: d;\
         \n}\n"
        );
    }
    #[test]
    fn next_sibling() {
        assert_eq!(
            runner().ok("a + b {c: d}\n"),
            "a + b {\
         \n  c: d;\
         \n}\n"
        );
    }
    #[test]
    fn sibling() {
        assert_eq!(
            runner().ok("a ~ b {c: d}\n"),
            "a ~ b {\
         \n  c: d;\
         \n}\n"
        );
    }
}
