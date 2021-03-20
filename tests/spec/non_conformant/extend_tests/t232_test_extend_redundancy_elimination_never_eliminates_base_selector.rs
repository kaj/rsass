//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/232_test_extend_redundancy_elimination_never_eliminates_base_selector.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "a.foo {a: b}\
            \n.foo {@extend a}\
            \n"
        )
        .unwrap(),
        "a.foo, .foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}
