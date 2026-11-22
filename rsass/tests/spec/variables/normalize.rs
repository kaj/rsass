//! Tests auto-converted from "sass-spec/spec/variables/normalize.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("normalize")
}

mod definition {
    use super::runner;

    #[test]
    fn direct() {
        assert_eq!(
            runner().ok("$a_b: c;\n\
             \nd {e: $a-b}\n"),
            "d {\
         \n  e: c;\
         \n}\n"
        );
    }
    #[test]
    fn escape() {
        assert_eq!(
            runner().ok("$a\\5f b: c;\n\
             \nd {e: $a-b}\n"),
            "d {\
         \n  e: c;\
         \n}\n"
        );
    }
}
mod test_use {
    use super::runner;

    #[test]
    fn direct() {
        assert_eq!(
            runner().ok("$a-b: c;\n\
             \nd {e: $a\\5f b}\n"),
            "d {\
         \n  e: c;\
         \n}\n"
        );
    }
    #[test]
    fn escape() {
        assert_eq!(
            runner().ok("$a-b: c;\n\
             \nd {e: $a\\5f b}\n"),
            "d {\
         \n  e: c;\
         \n}\n"
        );
    }
}
