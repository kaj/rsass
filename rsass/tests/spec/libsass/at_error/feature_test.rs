//! Tests auto-converted from "sass-spec/spec/libsass/at-error/feature-test.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("feature-test")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@if feature-exists(at-error) {\
             \n  div {\
             \n    feature: true;\
             \n  }\
             \n}\n"),
        "div {\
         \n  feature: true;\
         \n}\n"
    );
}
