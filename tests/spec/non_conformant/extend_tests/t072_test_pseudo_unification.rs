//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/072_test_pseudo_unification.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%-a ::foo.baz {a: b}\
            \n::foo {@extend .baz} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a ::foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}
