//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/104_test_basic_selector_interpolation.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo#{1 + 2} baz {a: b}\
            \n"
        )
        .unwrap(),
        "foo3 baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}
