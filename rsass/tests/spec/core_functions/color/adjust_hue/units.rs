//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust_hue/units.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("units")
}

#[test]
fn angle() {
    assert_eq!(
        runner().ok("a {b: adjust-hue(red, 60rad)}\n"),
        "a {\
         \n  b: rgb(0, 179.576224164, 255);\
         \n}\n"
    );
}
#[test]
fn deg() {
    assert_eq!(
        runner().ok("a {b: adjust-hue(red, 60deg)}\n"),
        "a {\
         \n  b: yellow;\
         \n}\n"
    );
}
#[test]
fn unitless() {
    assert_eq!(
        runner().ok("a {b: adjust-hue(red, 60)}\n"),
        "a {\
         \n  b: yellow;\
         \n}\n"
    );
}
#[test]
fn unknown() {
    assert_eq!(
        runner().ok("a {b: adjust-hue(red, 60in)}\n"),
        "a {\
         \n  b: yellow;\
         \n}\n"
    );
}
