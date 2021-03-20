//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/079_test_pseudoelement_remains_at_end_of_selector.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "a.foo::bar {a: b}\
            \n.baz {@extend .foo}\
            \n"
        )
        .unwrap(),
        "a.foo::bar, a.baz::bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
