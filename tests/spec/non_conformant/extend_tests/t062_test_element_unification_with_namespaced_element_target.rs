//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/062_test_element_unification_with_namespaced_element_target.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%-a ns|a.foo {a: b}\
            \na {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a ns|a.foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}
