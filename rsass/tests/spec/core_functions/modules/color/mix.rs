//! Tests auto-converted from "sass-spec/spec/core_functions/modules/color/mix.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mix")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.mix(#abcdef, #daddee)}\n"),
        "a {\
         \n  b: rgb(194.5, 213, 238.5);\
         \n}\n"
    );
}
