//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust_hue/middle.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("middle")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {b: adjust-hue(red, 123)}\n"),
        "a {\
         \n  b: rgb(0, 255, 12.75);\
         \n}\n"
    );
}
