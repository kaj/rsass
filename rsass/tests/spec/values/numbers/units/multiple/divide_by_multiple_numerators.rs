//! Tests auto-converted from "sass-spec/spec/values/numbers/units/multiple/divide_by_multiple_numerators.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("divide_by_multiple_numerators")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {\
             \n  b: meta.inspect(1 / (1px * 1rad));\
             \n}\n"),
        "a {\
         \n  b: calc(1 / 1px / 1rad);\
         \n}\n"
    );
}
