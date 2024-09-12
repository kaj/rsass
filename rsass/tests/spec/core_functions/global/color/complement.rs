//! Tests auto-converted from "sass-spec/spec/core_functions/global/color/complement.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("complement")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {b: complement(#abcdef)}\n"),
        "a {\
         \n  b: #efcdab;\
         \n}\n"
    );
}
