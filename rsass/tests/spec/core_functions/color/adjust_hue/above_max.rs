//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust_hue/above_max.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("above_max")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {b: adjust-hue(red, 540)}\n"),
        "a {\
         \n  b: aqua;\
         \n}\n"
    );
}
