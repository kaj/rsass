//! Tests auto-converted from "sass-spec/spec/css/url.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod escape {
    #[allow(unused)]
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
    #[ignore] // wrong result
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
