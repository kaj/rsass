//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2697.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2697")
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
