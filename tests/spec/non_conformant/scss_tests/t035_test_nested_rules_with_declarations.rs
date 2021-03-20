//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/035_test_nested_rules_with_declarations.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  a: b;\
            \n  bar {c: d}}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: b;\
        \n}\
        \nfoo bar {\
        \n  c: d;\
        \n}\
        \n"
    );
}
