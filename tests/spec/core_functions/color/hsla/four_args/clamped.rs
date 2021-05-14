//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsla/four_args/clamped.hrx"

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
            runner().ok("a {b: hsla(0, 100%, 50%, 1.1)}\n"),
            "a {\
         \n  b: red;\
         \n}\n"
        );
    }
    #[test]
    fn below() {
        assert_eq!(
            runner().ok("a {b: rgba(0, 100%, 50%, -0.1)}\n"),
            "a {\
         \n  b: rgba(0, 255, 128, 0);\
         \n}\n"
        );
    }
}
#[test]
fn lightness() {
    assert_eq!(
        runner().ok("a {b: hsla(0, 100%, 9999%, 0.5)}\n"),
        "a {\
         \n  b: rgba(255, 255, 255, 0.5);\
         \n}\n"
    );
}
#[test]
fn saturation() {
    assert_eq!(
        runner().ok("a {b: hsla(0, -0.1%, 50%, 0.5)}\n"),
        "a {\
         \n  b: rgba(128, 128, 128, 0.5);\
         \n}\n"
    );
}
