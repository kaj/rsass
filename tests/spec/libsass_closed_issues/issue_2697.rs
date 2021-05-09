//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2697.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".Card {\
             \n  &:not(.is-open, .is-static) {\
             \n    .CardContents {\
             \n      display: none;\
             \n    }\
             \n  }\
             \n}\n"),
        ".Card:not(.is-open, .is-static) .CardContents {\
         \n  display: none;\
         \n}\n"
    );
}
