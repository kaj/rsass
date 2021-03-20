//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/067_test_attribute_unification.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%-a [foo=bar].baz {a: b}\
            \n[foot=bar] {@extend .baz} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a [foo=bar].baz, -a [foo=bar][foot=bar] {\
        \n  a: b;\
        \n}\
        \n"
    );
}
