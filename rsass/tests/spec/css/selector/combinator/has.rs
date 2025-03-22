//! Tests auto-converted from "sass-spec/spec/css/selector/combinator/has.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("has")
}

mod leading {
    use super::runner;

    mod multiple {
        use super::runner;

        #[test]
        fn direct() {
            assert_eq!(runner().ok(":has(+ ~ a) {b: c}\n"), "");
        }
        #[test]
        fn parent() {
            assert_eq!(
                runner().ok("~ a {\
             \n  :has(+ &) {b: c}\
             \n}\n"),
                ""
            );
        }
    }
    mod single {
        use super::runner;

        #[test]
        fn child() {
            assert_eq!(
                runner().ok(":has(> a) {b: c}\n"),
                ":has(> a) {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        fn next_sibling() {
            assert_eq!(
                runner().ok(":has(+ a) {b: c}\n"),
                ":has(+ a) {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        fn parent() {
            assert_eq!(
                runner().ok("// Regression test for sass/sass#3546\
             \na {\
             \n  :has(~ &) {b: c}\
             \n}\n"),
                ":has(~ a) {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        fn sibling() {
            assert_eq!(
                runner().ok(":has(~ a) {b: c}\n"),
                ":has(~ a) {\
         \n  b: c;\
         \n}\n"
            );
        }
    }
}
