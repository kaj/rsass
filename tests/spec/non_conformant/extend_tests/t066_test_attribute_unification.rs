//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/066_test_attribute_unification.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%-a [foo=bar].baz {a: b}\
            \n[foo^=bar] {@extend .baz} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a [foo=bar].baz, -a [foo=bar][foo^=bar] {\
        \n  a: b;\
        \n}\
        \n"
    );
}
