//! Tests auto-converted from "sass-spec/spec/core_functions/global/color/fade-out.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("fade-out")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {b: fade-out(rgba(red, 0.5), 0.3)}\n"),
        "a {\
         \n  b: rgba(255, 0, 0, 0.2);\
         \n}\n"
    );
}
