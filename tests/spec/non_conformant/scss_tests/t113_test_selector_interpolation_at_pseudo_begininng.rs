//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/113_test_selector_interpolation_at_pseudo_begininng.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$zzz: zzz;\
            \n:#{$zzz}::#{$zzz} { a: b; }\
            \n"
        )
        .unwrap(),
        ":zzz::zzz {\
        \n  a: b;\
        \n}\
        \n"
    );
}
