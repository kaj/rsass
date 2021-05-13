//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/subject-operator/204_test_extend_with_subject_retains_subject_on_extender.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".foo .bar {a: b}\
             \n.bip! .bap {@extend .bar}\n"),
        ".foo .bar, .foo .bip! .bap, .bip! .foo .bap {\
         \n  a: b;\
         \n}\n"
    );
}
