//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/prop_name_only_interpolation.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {#{\"baz\" + \"bang\"}: blip}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bazbang: blip;\
        \n}\
        \n"
    );
}
