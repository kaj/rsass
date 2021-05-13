//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/subject-operator/205_test_extend_with_subject_fails_with_conflicting_subject.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("x! .bar {a: b}\
             \ny! .bap {@extend .bar}\n"),
        "x! .bar {\
         \n  a: b;\
         \n}\n"
    );
}
