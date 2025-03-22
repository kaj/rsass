//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/property_interpolation_at_dashes.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("property_interpolation_at_dashes")
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
