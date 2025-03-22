//! Tests auto-converted from "sass-spec/spec/values/numbers/units/multiple/multiplication_cancels_both.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("multiplication_cancels_both")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n$number: 1px * 1rad / 1ms / 1Hz;\
             \na {\
             \n  b: meta.inspect($number * (1ms / 1px));\
             \n}\n"),
        "a {\
         \n  b: calc(1rad / 1Hz);\
         \n}\n"
    );
}
