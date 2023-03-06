//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/four_args/in_gamut.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("in_gamut")
}

#[test]
fn named() {
    assert_eq!(
        runner().ok(
            "a {b: hsl($hue: 180, $saturation: 60%, $lightness: 50%, $alpha: 0.4)}\n"
        ),
        "a {\
         \n  b: hsla(180, 60%, 50%, 0.4);\
         \n}\n"
    );
}
#[test]
fn opaque() {
    assert_eq!(
        runner().ok("a {b: hsl(180, 60%, 50%, 1)}\n"),
        "a {\
         \n  b: hsl(180, 60%, 50%);\
         \n}\n"
    );
}
#[test]
fn partial() {
    assert_eq!(
        runner().ok("a {b: hsl(180, 60%, 50%, 0.5)}\n"),
        "a {\
         \n  b: hsla(180, 60%, 50%, 0.5);\
         \n}\n"
    );
}
#[test]
fn transparent() {
    assert_eq!(
        runner().ok("a {b: hsl(180, 60%, 50%, 0)}\n"),
        "a {\
         \n  b: hsla(180, 60%, 50%, 0);\
         \n}\n"
    );
}
