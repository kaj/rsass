//! Tests auto-converted from "sass-spec/spec/core_functions/global/color/hue.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hue")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {b: hue(#abcdef)}\n"),
        "a {\
         \n  b: 210deg;\
         \n}\n"
    );
}
