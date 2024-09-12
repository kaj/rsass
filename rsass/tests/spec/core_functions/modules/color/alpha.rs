//! Tests auto-converted from "sass-spec/spec/core_functions/modules/color/alpha.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("alpha")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.alpha(#abcdef)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
