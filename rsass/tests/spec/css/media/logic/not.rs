//! Tests auto-converted from "sass-spec/spec/css/media/logic/not.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("not")
}

mod not {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
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
    #[ignore] // wrong result
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
