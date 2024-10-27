//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust_hue/max.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("max")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {b: adjust-hue(red, 359)}\n"),
        "a {\
         \n  b: rgb(255, 0, 4.25);\
         \n}\n"
    );
}
