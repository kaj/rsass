//! Tests auto-converted from "sass-spec/spec/core_functions/global/color/opacity.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("opacity")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {b: opacity(#abcdef)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
