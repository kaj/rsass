//! Tests auto-converted from "sass-spec/spec/css/selector/combinator/has.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("has")
}

mod leading {
    #[allow(unused)]
    use super::runner;

    mod multiple {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn direct() {
            assert_eq!(runner().ok(":has(+ ~ a) {b: c}\n"), "");
        }
        #[test]
        #[ignore] // wrong result
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
        #[allow(unused)]
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
