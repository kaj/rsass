//! Tests auto-converted from "sass-spec/spec/css/selector/combinator/is.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("is")
}

mod leading {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn multiple() {
        assert_eq!(runner().ok(":is(+ ~ a) {b: c}\n"), "");
    }
    mod single {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn child() {
            assert_eq!(runner().ok(":is(> a) {b: c}\n"), "");
        }
        #[test]
        #[ignore] // wrong result
        fn next_sibling() {
            assert_eq!(runner().ok(":is(+ a) {b: c}\n"), "");
        }
        #[test]
        #[ignore] // wrong result
        fn sibling() {
            assert_eq!(runner().ok(":is(~ a) {b: c}\n"), "");
        }
    }
}
