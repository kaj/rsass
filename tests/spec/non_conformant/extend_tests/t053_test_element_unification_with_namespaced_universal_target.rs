//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/053_test_element_unification_with_namespaced_universal_target.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%-a ns|*.foo {a: b}\
            \na {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a ns|*.foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}
