//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/069_test_attribute_unification.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%-a %-a [foo=bar].bar {a: b}\
            \n[foo=bar] {@extend .bar} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a -a [foo=bar] {\
        \n  a: b;\
        \n}\
        \n"
    );
}
