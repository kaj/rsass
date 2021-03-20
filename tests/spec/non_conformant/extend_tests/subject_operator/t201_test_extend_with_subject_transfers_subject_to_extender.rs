//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/subject-operator/201_test_extend_with_subject_transfers_subject_to_extender.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "foo.x bar.y! baz.z {a: b}\
            \n.bip .bap {@extend .y}\
            \n"
        )
        .unwrap(),
        "foo.x bar.y! baz.z, foo.x .bip bar.bap! baz.z, .bip foo.x bar.bap! baz.z {\
        \n  a: b;\
        \n}\
        \n"
    );
}
