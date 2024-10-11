//! Tests auto-converted from "sass-spec/spec/core_functions/color/channel/positional.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("positional")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.channel(oklch(60% 70% 0.6turn), \"red\", rgb)}\n"),
        "a {\
         \n  b: -143.1028856642;\
         \n}\n"
    );
}
