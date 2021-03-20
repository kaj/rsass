//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/115_test_selector_interpolation_at_attr_end.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$zzz: zzz;\
            \n[foo=#{$zzz}] { a: b; }\
            \n"
        )
        .unwrap(),
        "[foo=zzz] {\
        \n  a: b;\
        \n}\
        \n"
    );
}
