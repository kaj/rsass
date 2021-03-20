//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/not-into-not-not.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "// Regression test for dart-sass#191.\
            \n:not(:not(.x)) {a: b}\
            \n:not(.y) {@extend .x}\
            \n"
        )
        .unwrap(),
        ":not(:not(.x)) {\
        \n  a: b;\
        \n}\
        \n"
    );
}
