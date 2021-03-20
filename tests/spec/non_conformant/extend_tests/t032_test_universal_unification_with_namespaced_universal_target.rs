//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/032_test_universal_unification_with_namespaced_universal_target.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%-a ns|*.foo {a: b}\
            \n*|* {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a ns|* {\
        \n  a: b;\
        \n}\
        \n"
    );
}
