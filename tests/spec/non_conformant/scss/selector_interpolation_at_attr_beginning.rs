//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/selector_interpolation_at_attr_beginning.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$zzz: zzz;\
            \n[#{$zzz}=foo] { a: b; }\
            \n"
        )
        .unwrap(),
        "[zzz=foo] {\
        \n  a: b;\
        \n}\
        \n"
    );
}
