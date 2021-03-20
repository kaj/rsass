//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/070_test_pseudo_unification.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%-a :foo.baz {a: b}\
            \n:foo(2n+1) {@extend .baz} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a :foo.baz, -a :foo:foo(2n+1) {\
        \n  a: b;\
        \n}\
        \n"
    );
}
