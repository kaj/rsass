//! Tests auto-converted from "sass-spec/spec/core_functions/color/is_powerless/space.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("space")
}

#[test]
#[ignore] // unexepected error
fn not_powerless() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.is-powerless(grey, \"a\", $space: lab)}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn powerless() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.is-powerless(grey, \"hue\", $space: hsl)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
