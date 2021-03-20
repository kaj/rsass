//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/038_test_nested_rules_with_fancy_selectors.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  .bar {a: b}\
            \n  :baz {c: d}\
            \n  bang:bop {e: f}}\
            \n"
        )
        .unwrap(),
        "foo .bar {\
        \n  a: b;\
        \n}\
        \nfoo :baz {\
        \n  c: d;\
        \n}\
        \nfoo bang:bop {\
        \n  e: f;\
        \n}\
        \n"
    );
}
