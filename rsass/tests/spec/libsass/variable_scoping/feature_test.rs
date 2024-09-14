//! Tests auto-converted from "sass-spec/spec/libsass/variable-scoping/feature-test.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("feature-test")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n@if meta.feature-exists(global-variable-shadowing) {\
             \n  div {\
             \n    feature: true;\
             \n  }\
             \n}"),
        "div {\
         \n  feature: true;\
         \n}\n"
    );
}
