//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/selector_interpolation_at_pseudo_begininng.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("selector_interpolation_at_pseudo_begininng")
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
