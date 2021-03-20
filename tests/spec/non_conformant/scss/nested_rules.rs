//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/nested_rules.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  bar {baz {a: b}}\
            \n  bang {bip {a: b}}}\
            \n"
        )
        .unwrap(),
        "foo bar baz {\
        \n  a: b;\
        \n}\
        \nfoo bang bip {\
        \n  a: b;\
        \n}\
        \n"
    );
}
