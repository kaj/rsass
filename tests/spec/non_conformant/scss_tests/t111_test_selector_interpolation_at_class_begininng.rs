//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/111_test_selector_interpolation_at_class_begininng.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$zzz: zzz;\
            \n.#{$zzz} { a: b; }\
            \n"
        )
        .unwrap(),
        ".zzz {\
        \n  a: b;\
        \n}\
        \n"
    );
}
