//! Tests auto-converted from "sass-spec/spec/css/selector/combinator/selector_pseudo.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("selector_pseudo")
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
mod middle {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn multiple() {
        assert_eq!(runner().ok(":is(a > + b) {c: d}\n"), "");
    }
    mod single {
        #[allow(unused)]
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
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn multiple() {
        assert_eq!(runner().ok(":is(a +) {b: c}\n"), "");
    }
    mod single {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn child() {
            assert_eq!(runner().ok(":is(a >) {b: c}\n"), "");
        }
        #[test]
        #[ignore] // wrong result
        fn next_sibling() {
            assert_eq!(runner().ok(":is(a +) {b: c}\n"), "");
        }
        #[test]
        #[ignore] // wrong result
        fn sibling() {
            assert_eq!(runner().ok(":is(a ~) {b: c}\n"), "");
        }
    }
}
