//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/087_test_negation_unification.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%-a :not(.foo).baz {a: b}\
            \n:not(.bar) {@extend .baz} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a :not(.foo).baz, -a :not(.foo):not(.bar) {\
        \n  a: b;\
        \n}\
        \n"
    );
}
