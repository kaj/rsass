//! Tests auto-converted from "sass-spec/spec/core_functions/color/invert/number.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("number")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.invert(1)}\n"),
        "a {\
         \n  b: invert(1);\
         \n}\n"
    );
}
