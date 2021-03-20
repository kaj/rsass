//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/112_test_selector_interpolation_at_id_begininng.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$zzz: zzz;\
            \n##{$zzz} { a: b; }\
            \n"
        )
        .unwrap(),
        "#zzz {\
        \n  a: b;\
        \n}\
        \n"
    );
}
