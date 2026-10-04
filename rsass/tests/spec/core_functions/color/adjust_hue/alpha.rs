//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust_hue/alpha.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("alpha")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("a {b: adjust-hue(rgba(red, 0.1), 359)}\n"),
        "a {\
         \n  b: rgba(100%, 0%, 1.6666666667%, 0.1);\
         \n}\n"
    );
}
