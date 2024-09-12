//! Tests auto-converted from "sass-spec/spec/core_functions/modules/color/complement.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("complement")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.complement(#abcdef)}\n"),
        "a {\
         \n  b: #efcdab;\
         \n}\n"
    );
}
