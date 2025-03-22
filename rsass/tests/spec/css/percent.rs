//! Tests auto-converted from "sass-spec/spec/css/percent.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("percent")
}

mod declaration {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn after() {
        assert_eq!(
            runner().ok("a {b: c %}\n"),
            "a {\
         \n  b: c %;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn alone() {
        assert_eq!(
            runner().ok("a {b: %}\n"),
            "a {\
         \n  b: %;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn before() {
        assert_eq!(
            runner().ok("a {b: % c}\n"),
            "a {\
         \n  b: % c;\
         \n}\n"
        );
    }
}
mod function {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn after() {
        assert_eq!(
            runner().ok("a {b: c(d %)}\n"),
            "a {\
         \n  b: c(d %);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn alone() {
        assert_eq!(
            runner().ok("a {b: c(%)}\n"),
            "a {\
         \n  b: c(%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn before() {
        assert_eq!(
            runner().ok("a {b: c(% d)}\n"),
            "a {\
         \n  b: c(% d);\
         \n}\n"
        );
    }
}
