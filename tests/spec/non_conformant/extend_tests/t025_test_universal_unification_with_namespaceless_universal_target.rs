//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/025_test_universal_unification_with_namespaceless_universal_target.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%-a *.foo {a: b}\
            \n* {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a * {\
        \n  a: b;\
        \n}\
        \n"
    );
}
