//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_gamut/positional.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("positional")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-gamut(pink, rgb, local-minde)}\n"),
        "a {\
         \n  b: pink;\
         \n}\n"
    );
}
