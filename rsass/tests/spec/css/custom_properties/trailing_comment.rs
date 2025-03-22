//! Tests auto-converted from "sass-spec/spec/css/custom_properties/trailing_comment.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("trailing_comment")
}

mod scss {
    use super::runner;

    #[test]
    fn loud() {
        assert_eq!(
            runner().ok("a {\
             \n  --b: c /* comment */;\
             \n}\n"),
            "a {\
         \n  --b: c /* comment */;\
         \n}\n"
        );
    }
    #[test]
    fn silent() {
        assert_eq!(
            runner().ok("a {\
             \n  --b: c // comment;\
             \n}\n"),
            "a {\
         \n  --b: c // comment;\
         \n}\n"
        );
    }
}
