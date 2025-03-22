//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/113_test_selector_interpolation_at_pseudo_begininng.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("113_test_selector_interpolation_at_pseudo_begininng")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$zzz: zzz;\
             \n:#{$zzz}::#{$zzz} { a: b; }\n"),
        ":zzz::zzz {\
         \n  a: b;\
         \n}\n"
    );
}
