//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust_hue/named.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("named")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {b: adjust-hue($color: red, $degrees: 123)}\n"),
        "a {\
         \n  b: rgb(0, 255, 12.75);\
         \n}\n"
    );
}
