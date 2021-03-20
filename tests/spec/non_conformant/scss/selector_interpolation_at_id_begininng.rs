//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/selector_interpolation_at_id_begininng.hrx"

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
