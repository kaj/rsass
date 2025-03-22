//! Tests auto-converted from "sass-spec/spec/values/numbers/divide/slash_free/value.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("value")
}

#[test]
fn inner_math() {
    assert_eq!(
        runner().ok("a {b: 1*1/2}\n"),
        "a {\
         \n  b: 0.5;\
         \n}\n"
    );
}
mod outer_math {
    use super::runner;

    #[test]
    fn left() {
        assert_eq!(
            runner().ok("a {b: 1+1/2}\n"),
            "a {\
         \n  b: 1.5;\
         \n}\n"
        );
    }
    #[test]
    fn right() {
        assert_eq!(
            runner().ok("a {b: 1/2+1}\n"),
            "a {\
         \n  b: 1.5;\
         \n}\n"
        );
    }
}
mod parentheses {
    use super::runner;

    #[test]
    fn all() {
        assert_eq!(
            runner().ok("a {b: (1/2)}\n"),
            "a {\
         \n  b: 0.5;\
         \n}\n"
        );
    }
    #[test]
    fn left() {
        assert_eq!(
            runner().ok("a {b: (1)/2}\n"),
            "a {\
         \n  b: 0.5;\
         \n}\n"
        );
    }
    #[test]
    fn right() {
        assert_eq!(
            runner().ok("a {b: 1/(2)}\n"),
            "a {\
         \n  b: 0.5;\
         \n}\n"
        );
    }
}
#[test]
fn parentheses_in_list() {
    assert_eq!(
        runner().ok("a {b: 3 (1/2) 4}\n"),
        "a {\
         \n  b: 3 0.5 4;\
         \n}\n"
    );
}
