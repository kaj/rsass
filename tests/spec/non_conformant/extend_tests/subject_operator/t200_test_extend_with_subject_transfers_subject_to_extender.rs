//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/subject-operator/200_test_extend_with_subject_transfers_subject_to_extender.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "foo bar! baz {a: b}\
            \n.bip .bap {@extend bar}\
            \n"
        )
        .unwrap(),
        "foo bar! baz, foo .bip .bap! baz, .bip foo .bap! baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}
