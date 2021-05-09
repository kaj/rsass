//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsla/four_args/in_gamut.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn named() {
    assert_eq!(
        runner().ok(
            "a {b: hsla($hue: 180, $saturation: 60%, $lightness: 50%, $alpha: 0.4)}\n"
        ),
        "a {\
         \n  b: rgba(51, 204, 204, 0.4);\
         \n}\n"
    );
}
#[test]
fn opaque() {
    assert_eq!(
        runner().ok("a {b: hsla(180, 60%, 50%, 1)}\n"),
        "a {\
         \n  b: #33cccc;\
         \n}\n"
    );
}
#[test]
fn partial() {
    assert_eq!(
        runner().ok("a {b: hsla(180, 60%, 50%, 0.5)}\n"),
        "a {\
         \n  b: rgba(51, 204, 204, 0.5);\
         \n}\n"
    );
}
#[test]
fn transparent() {
    assert_eq!(
        runner().ok("a {b: hsla(180, 60%, 50%, 0)}\n"),
        "a {\
         \n  b: rgba(51, 204, 204, 0);\
         \n}\n"
    );
}
