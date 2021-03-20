//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/basic_prop_name_interpolation.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {bar#{1 + 2}: blip}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar3: blip;\
        \n}\
        \n"
    );
}
