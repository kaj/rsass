//! Tests auto-converted from "sass-spec/spec/values/numbers/units/multiple/multiplication_cancels_compatible.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("multiplication_cancels_compatible")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$number: 1px * 1rad / 1ms / 1Hz;\
             \na {\
             \n  b: inspect($number * 1s);\
             \n}\n"),
        "a {\
         \n  b: 1000px*rad/Hz;\
         \n}\n"
    );
}
