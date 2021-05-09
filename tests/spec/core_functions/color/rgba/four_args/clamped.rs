//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgba/four_args/clamped.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod alpha {
    #[allow(unused)]
    use super::runner;
    #[test]
    fn above() {
        assert_eq!(
            runner().ok("a {b: rgba(0, 0, 0, 1.1)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    fn below() {
        assert_eq!(
            runner().ok("a {b: rgba(0, 0, 0, -0.1)}\n"),
            "a {\
         \n  b: rgba(0, 0, 0, 0);\
         \n}\n"
        );
    }
}
#[test]
fn blue() {
    assert_eq!(
        runner().ok("a {b: rgba(0, 0, 9999, 0.5)}\n"),
        "a {\
         \n  b: rgba(0, 0, 255, 0.5);\
         \n}\n"
    );
}
#[test]
fn green() {
    assert_eq!(
        runner().ok("a {b: rgba(0, -1, 0, 0.5)}\n"),
        "a {\
         \n  b: rgba(0, 0, 0, 0.5);\
         \n}\n"
    );
}
#[test]
fn red() {
    assert_eq!(
        runner().ok("a {b: rgba(256, 0, 0, 0.5)}\n"),
        "a {\
         \n  b: rgba(255, 0, 0, 0.5);\
         \n}\n"
    );
}
