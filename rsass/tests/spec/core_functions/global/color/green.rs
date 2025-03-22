//! Tests auto-converted from "sass-spec/spec/core_functions/global/color/green.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("green")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {b: green(#abcdef)}\n"),
        "a {\
         \n  b: 205;\
         \n}\n"
    );
}
