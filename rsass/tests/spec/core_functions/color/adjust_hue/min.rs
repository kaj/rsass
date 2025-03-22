//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust_hue/min.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("min")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {b: adjust-hue(blue, 0)}\n"),
        "a {\
         \n  b: blue;\
         \n}\n"
    );
}
