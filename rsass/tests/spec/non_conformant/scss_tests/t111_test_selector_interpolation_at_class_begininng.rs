//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/111_test_selector_interpolation_at_class_begininng.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("111_test_selector_interpolation_at_class_begininng")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$zzz: zzz;\
             \n.#{$zzz} { a: b; }\n"),
        ".zzz {\
         \n  a: b;\
         \n}\n"
    );
}
