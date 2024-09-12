//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/no_channels.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("no_channels")
}

#[test]
#[ignore] // unexepected error
fn identical() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.scale(oklch(50% 0.2 0deg), $space: lab)}\n"),
        "a {\
         \n  b: oklch(50% 0.2 0deg);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn missing() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.scale(rgb(none none none))}\n"),
        "a {\
         \n  b: rgb(none none none);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn powerless() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.scale(oklch(50% 0 0deg), $space: lab)}\n"),
        "a {\
         \n  b: oklch(50% 0 none);\
         \n}\n"
    );
}
