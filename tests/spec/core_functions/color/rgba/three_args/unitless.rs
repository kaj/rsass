//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgba/three_args/unitless.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn beaded() {
    assert_eq!(
        runner().ok("a {b: rgba(190, 173, 237)}\n"),
        "a {\
         \n  b: #beaded;\
         \n}\n"
    );
}
mod clamped {
    #[allow(unused)]
    use super::runner;
    #[test]
    fn blue() {
        assert_eq!(
            runner().ok("a {b: rgba(0, 0, 9999)}\n"),
            "a {\
         \n  b: blue;\
         \n}\n"
        );
    }
    #[test]
    fn green() {
        assert_eq!(
            runner().ok("a {b: rgba(0, -1, 0)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    fn red() {
        assert_eq!(
            runner().ok("a {b: rgba(256, 0, 0)}\n"),
            "a {\
         \n  b: red;\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("a {b: rgba($red: 0, $green: 255, $blue: 127)}\n"),
        "a {\
         \n  b: springgreen;\
         \n}\n"
    );
}
#[test]
fn numbers() {
    assert_eq!(
        runner().ok("a {b: rgba(18, 52, 86)}\n"),
        "a {\
         \n  b: #123456;\
         \n}\n"
    );
}
#[test]
fn springgreen() {
    assert_eq!(
        runner().ok("a {b: rgba(0, 255, 127)}\n"),
        "a {\
         \n  b: springgreen;\
         \n}\n"
    );
}
