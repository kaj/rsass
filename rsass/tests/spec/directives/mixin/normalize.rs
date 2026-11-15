//! Tests auto-converted from "sass-spec/spec/directives/mixin/normalize.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("normalize")
}

mod definition {
    use super::runner;

    #[test]
    fn direct() {
        assert_eq!(
            runner().ok("@mixin a_b {c: d}\n\
             \ne {@include a-b}\n"),
            "e {\
         \n  c: d;\
         \n}\n"
        );
    }
    #[test]
    fn escape() {
        assert_eq!(
            runner().ok("@mixin a\\5f b {c: d}\n\
             \ne {@include a-b}\n"),
            "e {\
         \n  c: d;\
         \n}\n"
        );
    }
}
mod test_use {
    use super::runner;

    #[test]
    fn direct() {
        assert_eq!(
            runner().ok("@mixin a-b {c: d}\n\
             \ne {@include a_b}\n"),
            "e {\
         \n  c: d;\
         \n}\n"
        );
    }
    #[test]
    fn escape() {
        assert_eq!(
            runner().ok("@mixin a-b {c: d}\n\
             \ne {@include a\\5f b}\n"),
            "e {\
         \n  c: d;\
         \n}\n"
        );
    }
}
