//! Tests auto-converted from "sass-spec/spec/core_functions/global/color/lightness.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lightness")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {b: lightness(#abcdef)}\n"),
        "a {\
         \n  b: 80.3921568627%;\
         \n}\n"
    );
}
