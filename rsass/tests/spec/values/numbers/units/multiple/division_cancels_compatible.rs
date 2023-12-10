//! Tests auto-converted from "sass-spec/spec/values/numbers/units/multiple/division_cancels_compatible.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("division_cancels_compatible")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$number: 1px * 1rad / 1ms / 1Hz;\
             \na {\
             \n  b: inspect($number / 1in);\
             \n}\n"),
        "a {\
         \n  b: calc(0.0104166667rad / 1ms / 1Hz);\
         \n}\n"
    );
}
