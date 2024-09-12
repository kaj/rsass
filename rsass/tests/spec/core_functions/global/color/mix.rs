//! Tests auto-converted from "sass-spec/spec/core_functions/global/color/mix.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mix")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("a {b: mix(#abcdef, #daddee)}\n"),
        "a {\
         \n  b: rgb(194.5, 213, 238.5);\
         \n}\n"
    );
}
