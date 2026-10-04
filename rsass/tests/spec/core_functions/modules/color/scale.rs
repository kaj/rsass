//! Tests auto-converted from "sass-spec/spec/core_functions/modules/color/scale.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("scale")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.scale(#abcdef, $red: 10%)}\n"),
        "a {\
         \n  b: rgb(70.3529411765%, 80.3921568627%, 93.7254901961%);\
         \n}\n"
    );
}
