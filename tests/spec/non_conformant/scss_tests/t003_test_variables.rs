//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/003_test_variables.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$var: foo;\
            \n\
            \nblat {a: $var}\
            \n"
        )
        .unwrap(),
        "blat {\
        \n  a: foo;\
        \n}\
        \n"
    );
}
