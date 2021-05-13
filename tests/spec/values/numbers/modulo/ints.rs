//! Tests auto-converted from "sass-spec/spec/values/numbers/modulo/ints.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod larger {
    #[allow(unused)]
    use super::runner;
    #[test]
    fn negative_negative() {
        assert_eq!(
            runner().ok("a {\
             \n  b: -7 % -5;\
             \n}\n"),
            "a {\
         \n  b: -2;\
         \n}\n"
        );
    }
    #[test]
    fn negative_positive() {
        assert_eq!(
            runner().ok("a {\
             \n  b: -7 % 5;\
             \n}\n"),
            "a {\
         \n  b: 3;\
         \n}\n"
        );
    }
    #[test]
    fn positive_negative() {
        assert_eq!(
            runner().ok("a {\
             \n  b: 6 % -5;\
             \n}"),
            "a {\
         \n  b: -4;\
         \n}\n"
        );
    }
    #[test]
    fn positive_positive() {
        assert_eq!(
            runner().ok("a {\
             \n  b: 6 % 5;\
             \n}\n"),
            "a {\
         \n  b: 1;\
         \n}\n"
        );
    }
}
#[test]
fn negative_negative() {
    assert_eq!(
        runner().ok("a {\
             \n  b: -1 % -4;\
             \n}\n"),
        "a {\
         \n  b: -1;\
         \n}\n"
    );
}
#[test]
fn negative_positive() {
    assert_eq!(
        runner().ok("a {\
             \n  b: -1 % 4;\
             \n}\n"),
        "a {\
         \n  b: 3;\
         \n}\n"
    );
}
#[test]
fn positive_negative() {
    assert_eq!(
        runner().ok("a {\
             \n  b: 1 % -4;\
             \n}"),
        "a {\
         \n  b: -3;\
         \n}\n"
    );
}
#[test]
fn positive_positive() {
    assert_eq!(
        runner().ok("a {\
             \n  b: 1 % 4;\
             \n}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
