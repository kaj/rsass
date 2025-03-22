//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/selector_interpolation_at_id_begininng.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("selector_interpolation_at_id_begininng")
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
