//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/230_test_extend_redundancy_elimination_when_it_would_reduce_specificity.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "a {a: b}\
            \na.foo {@extend a}\
            \n"
        )
        .unwrap(),
        "a, a.foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}
