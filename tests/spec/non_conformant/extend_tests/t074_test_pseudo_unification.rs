//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/074_test_pseudo_unification.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%-a :foo.baz {a: b}\
            \n:bar {@extend .baz} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a :foo.baz, -a :foo:bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
