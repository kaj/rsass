//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/four_args/clamped.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("clamped")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn above() {
        assert_eq!(
            runner().ok("a {b: hsl(0, 100%, 50%, 1.1)}\n"),
            "a {\
         \n  b: hsl(0, 100%, 50%);\
         \n}\n"
        );
    }
    #[test]
    fn below() {
        assert_eq!(
            runner().ok("a {b: hsla(0, 100%, 50%, -0.1)}\n"),
            "a {\
         \n  b: hsla(0, 100%, 50%, 0);\
         \n}\n"
        );
    }
}
#[test]
fn blue() {
    assert_eq!(
        runner().ok("a {b: hsl(0, 100%, 9999%, 0.5)}\n"),
        "a {\
         \n  b: hsla(0, 100%, 100%, 0.5);\
         \n}\n"
    );
}
#[test]
fn saturation() {
    assert_eq!(
        runner().ok("a {b: hsl(0, -0.1%, 50%, 0.5)}\n"),
        "a {\
         \n  b: hsla(0, 0%, 50%, 0.5);\
         \n}\n"
    );
}
