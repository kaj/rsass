//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/055_test_element_unification_with_namespaced_universal_target.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%-a ns|*.foo {a: b}\
            \nns|a {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a ns|*.foo, -a ns|a {\
        \n  a: b;\
        \n}\
        \n"
    );
}
