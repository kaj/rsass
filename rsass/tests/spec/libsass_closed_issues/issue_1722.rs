//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1722.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1722")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$score: (item-height: 1.12em);\
             \n.test {\
             \n    background-position: 0 -#{map-get($score, item-height)};\
             \n}\n\n"),
        ".test {\
         \n  background-position: 0 -1.12em;\
         \n}\n"
    );
}
