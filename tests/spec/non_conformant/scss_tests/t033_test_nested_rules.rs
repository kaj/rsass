//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/033_test_nested_rules.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  bar {a: b}\
            \n  baz {b: c}}\
            \n"
        )
        .unwrap(),
        "foo bar {\
        \n  a: b;\
        \n}\
        \nfoo baz {\
        \n  b: c;\
        \n}\
        \n"
    );
}
