//! Tests auto-converted from "sass-spec/spec/values/numbers/units/multiple/division_cancels_both.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("division_cancels_both")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n$number: 1px * 1rad / 1ms / 1Hz;\
             \na {\
             \n  b: meta.inspect($number / (1px / 1ms));\
             \n}\n"),
        "a {\
         \n  b: calc(1rad / 1Hz);\
         \n}\n"
    );
}
