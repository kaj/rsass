//! Tests auto-converted from "sass-spec/spec/css/selector/combinator/selector_pseudo.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("selector_pseudo")
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
mod middle {
    use super::runner;

    #[test]
    fn multiple() {
        assert_eq!(runner().ok(":is(a > + b) {c: d}\n"), "");
    }
    mod single {
        use super::runner;

        #[test]
        fn child() {
            assert_eq!(
                runner().ok(":is(a > b) {c: d}\n"),
                ":is(a > b) {\
         \n  c: d;\
         \n}\n"
            );
        }
        #[test]
        fn next_sibling() {
            assert_eq!(
                runner().ok(":is(a + b) {c: d}\n"),
                ":is(a + b) {\
         \n  c: d;\
         \n}\n"
            );
        }
        #[test]
        fn sibling() {
            assert_eq!(
                runner().ok(":is(a ~ b) {c: d}\n"),
                ":is(a ~ b) {\
         \n  c: d;\
         \n}\n"
            );
        }
    }
}
mod trailing {
    use super::runner;

    #[test]
    fn multiple() {
        assert_eq!(runner().ok(":is(a +) {b: c}\n"), "");
    }
    mod single {
        use super::runner;

        #[test]
        fn child() {
            assert_eq!(runner().ok(":is(a >) {b: c}\n"), "");
        }
        #[test]
        fn next_sibling() {
            assert_eq!(runner().ok(":is(a +) {b: c}\n"), "");
        }
        #[test]
        fn sibling() {
            assert_eq!(runner().ok(":is(a ~) {b: c}\n"), "");
        }
    }
}
