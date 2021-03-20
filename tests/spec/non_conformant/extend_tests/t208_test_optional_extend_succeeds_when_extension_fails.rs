//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/208_test_optional_extend_succeeds_when_extension_fails.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "a.bar {a: b}\
            \nb.foo {@extend .bar !optional}\
            \n"
        )
        .unwrap(),
        "a.bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
