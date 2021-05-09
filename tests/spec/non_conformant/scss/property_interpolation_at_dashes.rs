//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/property_interpolation_at_dashes.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$a : a;\
             \n$b : b;\
             \ndiv { -foo-#{$a}-#{$b}-foo: foo }\n"),
        "div {\
         \n  -foo-a-b-foo: foo;\
         \n}\n"
    );
}
