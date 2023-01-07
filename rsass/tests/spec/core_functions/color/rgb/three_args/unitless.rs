//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/three_args/unitless.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("unitless")
}

#[test]
fn beaded() {
    assert_eq!(
        runner().ok("a {b: rgb(190, 173, 237)}\n"),
        "a {\
         \n  b: rgb(190, 173, 237);\
         \n}\n"
    );
}
mod clamped {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn blue() {
        assert_eq!(
            runner().ok("a {b: rgb(0, 0, 9999)}\n"),
            "a {\
         \n  b: rgb(0, 0, 255);\
         \n}\n"
        );
    }
    #[test]
    fn green() {
        assert_eq!(
            runner().ok("a {b: rgb(0, -1, 0)}\n"),
            "a {\
         \n  b: rgb(0, 0, 0);\
         \n}\n"
        );
    }
    #[test]
    fn red() {
        assert_eq!(
            runner().ok("a {b: rgb(256, 0, 0)}\n"),
            "a {\
         \n  b: rgb(255, 0, 0);\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("a {b: rgb($red: 0, $green: 255, $blue: 127)}\n"),
        "a {\
         \n  b: rgb(0, 255, 127);\
         \n}\n"
    );
}
#[test]
fn numbers() {
    assert_eq!(
        runner().ok("a {b: rgb(18, 52, 86)}\n"),
        "a {\
         \n  b: rgb(18, 52, 86);\
         \n}\n"
    );
}
#[test]
fn springgreen() {
    assert_eq!(
        runner().ok("a {b: rgb(0, 255, 127)}\n"),
        "a {\
         \n  b: rgb(0, 255, 127);\
         \n}\n"
    );
}
