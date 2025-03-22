//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/115_test_selector_interpolation_at_attr_end.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("115_test_selector_interpolation_at_attr_end")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$zzz: zzz;\
             \n[foo=#{$zzz}] { a: b; }\n"),
        "[foo=zzz] {\
         \n  a: b;\
         \n}\n"
    );
}
