//! Tests auto-converted from "sass-spec/spec/core_functions/global/color/alpha.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("alpha")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {b: alpha(#abcdef)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
