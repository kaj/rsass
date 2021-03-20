//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/041_test_universal_unification_with_namespaced_element_target.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%-a ns|a.foo {a: b}\
            \n*|* {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a ns|a {\
        \n  a: b;\
        \n}\
        \n"
    );
}
