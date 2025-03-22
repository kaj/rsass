//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/114_test_selector_interpolation_at_attr_beginning.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("114_test_selector_interpolation_at_attr_beginning")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$zzz: zzz;\
             \n[#{$zzz}=foo] { a: b; }\n"),
        "[zzz=foo] {\
         \n  a: b;\
         \n}\n"
    );
}
