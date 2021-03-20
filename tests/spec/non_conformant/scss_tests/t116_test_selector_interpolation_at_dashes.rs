//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/116_test_selector_interpolation_at_dashes.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$a : a;\
            \n$b : b;\
            \ndiv { -foo-#{$a}-#{$b}-foo: foo }\
            \n"
        )
        .unwrap(),
        "div {\
        \n  -foo-a-b-foo: foo;\
        \n}\
        \n"
    );
}
