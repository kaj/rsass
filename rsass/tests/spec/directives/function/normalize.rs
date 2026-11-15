//! Tests auto-converted from "sass-spec/spec/directives/function/normalize.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("normalize")
}

mod definition {
    use super::runner;

    #[test]
    fn direct() {
        assert_eq!(
            runner().ok("@function a_b() {@return c}\n\
             \nd {e: a-b()}\n"),
            "d {\
         \n  e: c;\
         \n}\n"
        );
    }
    #[test]
    fn escape() {
        assert_eq!(
            runner().ok("@function a\\5f b() {@return c}\n\
             \nd {e: a-b()}\n"),
            "d {\
         \n  e: c;\
         \n}\n"
        );
    }
}
mod test_use {
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn direct() {
        assert_eq!(
            runner().ok("@function a-b() {@return c}\n\
             \nd {e: a\\5f b()}\n"),
            "d {\
         \n  e: c;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn escape() {
        assert_eq!(
            runner().ok("@function a-b() {@return c}\n\
             \nd {e: a\\5f b()}\n"),
            "d {\
         \n  e: c;\
         \n}\n"
        );
    }
}
