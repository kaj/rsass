//! Tests auto-converted from "sass-spec/spec/css/url.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("url")
}

mod escape {
    use super::runner;

    #[test]
    fn ascii() {
        assert_eq!(
            runner().ok("a {b: url(\\41)}\n"),
            "a {\
         \n  b: url(A);\
         \n}\n"
        );
    }
    #[test]
    fn close_paren() {
        assert_eq!(
            runner().ok("a {b: url(\\))}\n"),
            "a {\
         \n  b: url(\\));\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn interpolation() {
        assert_eq!(
            runner().ok("a {b: url(\\#{})}\n"),
            "a {\
         \n  b: url(\\#{});\
         \n}\n"
        );
    }
    #[test]
    fn non_ascii() {
        assert_eq!(
            runner().ok("a {b: url(\\2603)}\n"),
            "@charset \"UTF-8\";\
         \na {\
         \n  b: url(☃);\
         \n}\n"
        );
    }
}
