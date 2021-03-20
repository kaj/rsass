//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/105_test_basic_selector_interpolation.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo#{\".bar\"} baz {a: b}\
            \n"
        )
        .unwrap(),
        "foo.bar baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}
