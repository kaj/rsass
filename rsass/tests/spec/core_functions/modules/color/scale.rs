//! Tests auto-converted from "sass-spec/spec/core_functions/modules/color/scale.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("scale")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.scale(#abcdef, $red: 10%)}\n"),
        "a {\
         \n  b: rgb(179.4, 205, 239);\
         \n}\n"
    );
}
