//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/112_test_selector_interpolation_at_id_begininng.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("112_test_selector_interpolation_at_id_begininng")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$zzz: zzz;\
             \n##{$zzz} { a: b; }\n"),
        "#zzz {\
         \n  a: b;\
         \n}\n"
    );
}
