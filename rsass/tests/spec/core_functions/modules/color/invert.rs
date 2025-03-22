//! Tests auto-converted from "sass-spec/spec/core_functions/modules/color/invert.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("invert")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.invert(#abcdef)}\n"),
        "a {\
         \n  b: #543210;\
         \n}\n"
    );
}
