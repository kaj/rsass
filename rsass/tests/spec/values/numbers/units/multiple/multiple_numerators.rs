//! Tests auto-converted from "sass-spec/spec/values/numbers/units/multiple/multiple_numerators.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("multiple_numerators")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {\
             \n  b: meta.inspect(1px * 1rad);\
             \n}\n"),
        "a {\
         \n  b: calc(1px * 1rad);\
         \n}\n"
    );
}
