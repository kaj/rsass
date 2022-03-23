//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/four_args/in_gamut.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn named() {
    assert_eq!(
        runner().ok(
            "a {b: rgb($red: 0, $green: 255, $blue: 127, $alpha: 0.3)}\n"
        ),
        "a {\
         \n  b: rgba(0, 255, 127, 0.3);\
         \n}\n"
    );
}
#[test]
fn opaque() {
    assert_eq!(
        runner().ok("a {b: rgb(190, 173, 237, 1)}\n"),
        "a {\
         \n  b: rgb(190, 173, 237);\
         \n}\n"
    );
}
#[test]
fn partial() {
    assert_eq!(
        runner().ok("a {b: rgb(18, 52, 86, 0.5)}\n"),
        "a {\
         \n  b: rgba(18, 52, 86, 0.5);\
         \n}\n"
    );
}
#[test]
fn transparent() {
    assert_eq!(
        runner().ok("a {b: rgb(0, 255, 127, 0)}\n"),
        "a {\
         \n  b: rgba(0, 255, 127, 0);\
         \n}\n"
    );
}
