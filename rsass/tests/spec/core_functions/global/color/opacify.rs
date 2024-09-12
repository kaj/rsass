//! Tests auto-converted from "sass-spec/spec/core_functions/global/color/opacify.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("opacify")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {b: opacify(rgba(red, 0.5), 0.3)}\n"),
        "a {\
         \n  b: rgba(255, 0, 0, 0.8);\
         \n}\n"
    );
}
