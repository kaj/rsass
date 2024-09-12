//! Tests auto-converted from "sass-spec/spec/core_functions/global/color/scale.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("scale")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("a {b: scale-color(#abcdef, $red: 10%)}\n"),
        "a {\
         \n  b: rgb(179.4, 205, 239);\
         \n}\n"
    );
}
