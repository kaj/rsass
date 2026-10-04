//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust_hue/max.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("max")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("a {b: adjust-hue(red, 359)}\n"),
        "a {\
         \n  b: rgb(100%, 0%, 1.6666666667%);\
         \n}\n"
    );
}
