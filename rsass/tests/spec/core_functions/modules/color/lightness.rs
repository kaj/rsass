//! Tests auto-converted from "sass-spec/spec/core_functions/modules/color/lightness.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lightness")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.lightness(#abcdef)}\n"),
        "a {\
         \n  b: 80.3921568627%;\
         \n}\n"
    );
}
