//! Tests auto-converted from "sass-spec/spec/core_functions/color/mix/mixed_spaces.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mixed_spaces")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("a {b: mix(hsl(0 100% 50%), green, $method: lch)}\n"),
        "a {\
         \n  b: hsl(50.9351301875, 199.2813015981%, 19.0269267557%);\
         \n}\n"
    );
}
