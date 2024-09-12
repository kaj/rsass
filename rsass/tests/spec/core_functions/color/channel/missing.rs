//! Tests auto-converted from "sass-spec/spec/core_functions/color/channel/missing.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("missing")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.channel(rgb(255 none 255), \"green\")}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
