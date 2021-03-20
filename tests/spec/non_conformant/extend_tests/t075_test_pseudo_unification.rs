//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/075_test_pseudo_unification.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%-a .baz:foo {a: b}\
            \n:after {@extend .baz} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a .baz:foo, -a :foo:after {\
        \n  a: b;\
        \n}\
        \n"
    );
}
