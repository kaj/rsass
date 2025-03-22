//! Tests auto-converted from "sass-spec/spec/core_functions/color/invert/alpha.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("alpha")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.invert(rgba(turquoise, 0.4))}\n"),
        "a {\
         \n  b: rgba(191, 31, 47, 0.4);\
         \n}\n"
    );
}
