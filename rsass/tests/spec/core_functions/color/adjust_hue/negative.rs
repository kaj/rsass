//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust_hue/negative.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("negative")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {b: adjust-hue(red, -180)}\n"),
        "a {\
         \n  b: aqua;\
         \n}\n"
    );
}
