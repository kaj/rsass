//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/prop_name_interpolation_after_hyphen.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "a { -#{\"foo\"}-bar: b; }\
            \n"
        )
        .unwrap(),
        "a {\
        \n  -foo-bar: b;\
        \n}\
        \n"
    );
}
