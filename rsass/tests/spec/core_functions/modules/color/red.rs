//! Tests auto-converted from "sass-spec/spec/core_functions/modules/color/red.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("red")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.red(#abcdef)}\n"),
        "a {\
         \n  b: 171;\
         \n}\n"
    );
}
