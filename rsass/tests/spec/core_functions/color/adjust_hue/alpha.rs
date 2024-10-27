//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust_hue/alpha.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("alpha")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {b: adjust-hue(rgba(red, 0.1), 359)}\n"),
        "a {\
         \n  b: rgba(255, 0, 4.25, 0.1);\
         \n}\n"
    );
}
