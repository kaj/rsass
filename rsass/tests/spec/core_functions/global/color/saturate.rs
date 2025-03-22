//! Tests auto-converted from "sass-spec/spec/core_functions/global/color/saturate.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("saturate")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {b: saturate(#abcdef, 10%)}\n"),
        "a {\
         \n  b: #a6cdf4;\
         \n}\n"
    );
}
