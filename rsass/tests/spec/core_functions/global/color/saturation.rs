//! Tests auto-converted from "sass-spec/spec/core_functions/global/color/saturation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("saturation")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {b: saturation(#abcdef)}\n"),
        "a {\
         \n  b: 68%;\
         \n}\n"
    );
}
