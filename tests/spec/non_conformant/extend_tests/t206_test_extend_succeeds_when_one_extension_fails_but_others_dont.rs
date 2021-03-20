//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/206_test_extend_succeeds_when_one_extension_fails_but_others_dont.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "a.bar {a: b}\
            \n.bar {c: d}\
            \nb.foo {@extend .bar}\
            \n"
        )
        .unwrap(),
        "a.bar {\
        \n  a: b;\
        \n}\
        \n.bar, b.foo {\
        \n  c: d;\
        \n}\
        \n"
    );
}
