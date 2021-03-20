//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/231_test_extend_redundancy_elimination_when_it_would_preserve_specificity.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".bar a {a: b}\
            \na.foo {@extend a}\
            \n"
        )
        .unwrap(),
        ".bar a {\
        \n  a: b;\
        \n}\
        \n"
    );
}
