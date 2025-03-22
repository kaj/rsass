//! Tests auto-converted from "sass-spec/spec/css/selector/combinator/is.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("is")
}

mod leading {
    use super::runner;

    #[test]
    fn multiple() {
        assert_eq!(runner().ok(":is(+ ~ a) {b: c}\n"), "");
    }
    mod single {
        use super::runner;

        #[test]
        fn child() {
            assert_eq!(runner().ok(":is(> a) {b: c}\n"), "");
        }
        #[test]
        fn next_sibling() {
            assert_eq!(runner().ok(":is(+ a) {b: c}\n"), "");
        }
        #[test]
        fn sibling() {
            assert_eq!(runner().ok(":is(~ a) {b: c}\n"), "");
        }
    }
}
