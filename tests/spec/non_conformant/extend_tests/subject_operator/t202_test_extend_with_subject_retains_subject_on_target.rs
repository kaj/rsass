//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/subject-operator/202_test_extend_with_subject_retains_subject_on_target.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo! .bar {a: b}\
            \n.bip .bap {@extend .bar}\
            \n"
        )
        .unwrap(),
        ".foo! .bar, .foo! .bip .bap, .bip .foo! .bap {\
        \n  a: b;\
        \n}\
        \n"
    );
}
