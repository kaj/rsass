//! Tests auto-converted from "sass-spec/spec/core_functions/global/color/fade-in.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("fade-in")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {b: fade-in(rgba(red, 0.5), 0.3)}\n"),
        "a {\
         \n  b: rgba(255, 0, 0, 0.8);\
         \n}\n"
    );
}
