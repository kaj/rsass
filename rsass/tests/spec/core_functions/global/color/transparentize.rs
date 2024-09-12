//! Tests auto-converted from "sass-spec/spec/core_functions/global/color/transparentize.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("transparentize")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {b: transparentize(rgba(red, 0.5), 0.3)}\n"),
        "a {\
         \n  b: rgba(255, 0, 0, 0.2);\
         \n}\n"
    );
}
