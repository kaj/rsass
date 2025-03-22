//! Tests auto-converted from "sass-spec/spec/core_functions/modules/color/green.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("green")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.green(#abcdef)}\n"),
        "a {\
         \n  b: 205;\
         \n}\n"
    );
}
