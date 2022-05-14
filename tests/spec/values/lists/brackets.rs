//! Tests auto-converted from "sass-spec/spec/values/lists/brackets.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("brackets")
}

#[test]
fn empty() {
    assert_eq!(
        runner().ok("a {b: []}\n"),
        "a {\
         \n  b: [];\
         \n}\n"
    );
}
#[test]
fn multiple() {
    assert_eq!(
        runner().ok("a {b: [c d]}\n"),
        "a {\
         \n  b: [c d];\
         \n}\n"
    );
}
mod nested {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn empty() {
        assert_eq!(
            runner().ok("a {b: [[]]}\n"),
            "a {\
         \n  b: [[]];\
         \n}\n"
        );
    }
    #[test]
    fn multiple() {
        assert_eq!(
            runner().ok("a {b: [[c] [d]]}\n"),
            "a {\
         \n  b: [[c] [d]];\
         \n}\n"
        );
    }
    #[test]
    fn single() {
        assert_eq!(
            runner().ok("a {b: [[c]]}\n"),
            "a {\
         \n  b: [[c]];\
         \n}\n"
        );
    }
    mod unbracketed {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn empty() {
            assert_eq!(
                runner().ok("a {b: [()]}\n"),
                "a {\
         \n  b: [];\
         \n}\n"
            );
        }
        #[test]
        fn multiple() {
            assert_eq!(
                runner().ok("a {b: [(c,) (d e)]}\n"),
                "a {\
         \n  b: [c d e];\
         \n}\n"
            );
        }
        #[test]
        fn single() {
            assert_eq!(
                runner().ok("a {b: [(c,)]}\n"),
                "a {\
         \n  b: [c];\
         \n}\n"
            );
        }
    }
}
#[test]
fn single() {
    assert_eq!(
        runner().ok("a {b: [c]}\n"),
        "a {\
         \n  b: [c];\
         \n}\n"
    );
}
