//! Tests auto-converted from "sass-spec/spec/css/media/logic/not.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("not")
}

mod not {
    use super::runner;

    #[test]
    fn comment_after() {
        assert_eq!(
            runner().ok("@media not/**/(a) {x {y: z}}\n"),
            "@media not (a) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
        );
    }
    #[test]
    fn interpolation() {
        assert_eq!(
            runner().ok("@media not #{\"(a)\"} {x {y: z}}\n"),
            "@media not (a) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
        );
    }
    #[test]
    fn mixed_case() {
        assert_eq!(
            runner().ok("@media NoT (a) {x {y: z}}\n"),
            "@media not (a) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
        );
    }
    #[test]
    fn parens() {
        assert_eq!(
            runner().ok("@media not (a) {x {y: z}}\n"),
            "@media not (a) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
        );
    }
}
