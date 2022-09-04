//! Tests auto-converted from "sass-spec/spec/values/numbers/units/multiple/multiple_denominators.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("multiple_denominators")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {\
             \n  b: inspect((1 / 1px / 1rad));\
             \n}\n"),
        "a {\
         \n  b: 1(px*rad)^-1;\
         \n}\n"
    );
}
