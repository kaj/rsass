//! Tests auto-converted from "sass-spec/spec/core_functions/modules/color/saturation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("saturation")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.saturation(#abcdef)}\n"),
        "a {\
         \n  b: 68%;\
         \n}\n"
    );
}
