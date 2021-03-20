//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/089_test_negation_unification.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%-a :not([a=b]).baz {a: b}\
            \n:not([a = b]) {@extend .baz} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a :not([a=b]) {\
        \n  a: b;\
        \n}\
        \n"
    );
}
