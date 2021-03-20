//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/subject-operator/205_test_extend_with_subject_fails_with_conflicting_subject.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "x! .bar {a: b}\
            \ny! .bap {@extend .bar}\
            \n"
        )
        .unwrap(),
        "x! .bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
