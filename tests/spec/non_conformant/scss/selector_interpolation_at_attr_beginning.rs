//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/selector_interpolation_at_attr_beginning.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
