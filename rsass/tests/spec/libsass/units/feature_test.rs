//! Tests auto-converted from "sass-spec/spec/libsass/units/feature-test.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("feature-test")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@if feature-exists(units-level-3) {\
             \n  div {\
             \n    feature: true;\
             \n  }\
             \n}\n"),
        "div {\
         \n  feature: true;\
         \n}\n"
    );
}
