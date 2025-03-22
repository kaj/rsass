//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/selector_interpolation_at_attr_beginning.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("selector_interpolation_at_attr_beginning")
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
