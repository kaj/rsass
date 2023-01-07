//! Tests auto-converted from "sass-spec/spec/values/numbers/units/multiple/multiplication_cancels_numerator.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("multiplication_cancels_numerator")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$number: 1px * 1rad / 1ms / 1Hz;\
             \na {\
             \n  b: inspect($number * (1 / 1px));\
             \n}\n"),
        "a {\
         \n  b: 1rad/ms*Hz;\
         \n}\n"
    );
}
