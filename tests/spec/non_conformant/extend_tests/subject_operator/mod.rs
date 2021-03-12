//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/subject-operator"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/extend-tests/subject-operator/200_test_extend_with_subject_transfers_subject_to_extender.hrx"
#[test]
#[ignore] // unexepected error
fn t200_test_extend_with_subject_transfers_subject_to_extender() {
    assert_eq!(
        rsass(
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

// From "sass-spec/spec/non_conformant/extend-tests/subject-operator/201_test_extend_with_subject_transfers_subject_to_extender.hrx"
#[test]
#[ignore] // unexepected error
fn t201_test_extend_with_subject_transfers_subject_to_extender() {
    assert_eq!(
        rsass(
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

// From "sass-spec/spec/non_conformant/extend-tests/subject-operator/202_test_extend_with_subject_retains_subject_on_target.hrx"
#[test]
#[ignore] // unexepected error
fn t202_test_extend_with_subject_retains_subject_on_target() {
    assert_eq!(
        rsass(
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

// From "sass-spec/spec/non_conformant/extend-tests/subject-operator/203_test_extend_with_subject_transfers_subject_to_target.hrx"

// Ignoring "t203_test_extend_with_subject_transfers_subject_to_target", error tests are not supported yet.

// From "sass-spec/spec/non_conformant/extend-tests/subject-operator/204_test_extend_with_subject_retains_subject_on_extender.hrx"
#[test]
#[ignore] // unexepected error
fn t204_test_extend_with_subject_retains_subject_on_extender() {
    assert_eq!(
        rsass(
            ".foo .bar {a: b}\
            \n.bip! .bap {@extend .bar}\
            \n"
        )
        .unwrap(),
        ".foo .bar, .foo .bip! .bap, .bip! .foo .bap {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/subject-operator/205_test_extend_with_subject_fails_with_conflicting_subject.hrx"
#[test]
#[ignore] // unexepected error
fn t205_test_extend_with_subject_fails_with_conflicting_subject() {
    assert_eq!(
        rsass(
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
