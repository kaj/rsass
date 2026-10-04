//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust_hue/fraction.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("fraction")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("a {b: adjust-hue(red, 0.5)}\n"),
        "a {\
         \n  b: rgb(100%, 0.8333333333%, 0%);\
         \n}\n"
    );
}
