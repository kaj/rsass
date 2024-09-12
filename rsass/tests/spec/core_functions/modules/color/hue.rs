//! Tests auto-converted from "sass-spec/spec/core_functions/modules/color/hue.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hue")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.hue(#abcdef)}\n"),
        "a {\
         \n  b: 210deg;\
         \n}\n"
    );
}
