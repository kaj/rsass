//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/115_test_selector_interpolation_at_attr_end.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
