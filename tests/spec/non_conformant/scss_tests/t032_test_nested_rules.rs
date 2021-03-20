//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/032_test_nested_rules.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {bar {a: b}}\
            \n"
        )
        .unwrap(),
        "foo bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
