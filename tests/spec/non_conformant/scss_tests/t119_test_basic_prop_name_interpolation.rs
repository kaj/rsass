//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/119_test_basic_prop_name_interpolation.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {bar#{\"baz\" + \"bang\"}: blip}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  barbazbang: blip;\
        \n}\
        \n"
    );
}
