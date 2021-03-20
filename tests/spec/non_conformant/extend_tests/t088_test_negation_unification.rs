//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/088_test_negation_unification.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%-a :not(.foo).baz {a: b}\
            \n:not(.foo) {@extend .baz} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a :not(.foo) {\
        \n  a: b;\
        \n}\
        \n"
    );
}
