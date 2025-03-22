//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust_hue/fraction.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("fraction")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {b: adjust-hue(red, 0.5)}\n"),
        "a {\
         \n  b: rgb(255, 2.125, 0);\
         \n}\n"
    );
}
