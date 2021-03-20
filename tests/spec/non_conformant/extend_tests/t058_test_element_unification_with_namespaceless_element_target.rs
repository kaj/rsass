//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/058_test_element_unification_with_namespaceless_element_target.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%-a *|a.foo {a: b}\
            \na {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a *|a.foo, -a a {\
        \n  a: b;\
        \n}\
        \n"
    );
}
