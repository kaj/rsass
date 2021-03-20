//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/060_test_element_unification_with_namespaceless_element_target.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%-a a.foo {a: b}\
            \nns|a {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a a.foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}
