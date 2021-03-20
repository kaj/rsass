//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2697.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".Card {\
            \n  &:not(.is-open, .is-static) {\
            \n    .CardContents {\
            \n      display: none;\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".Card:not(.is-open, .is-static) .CardContents {\
        \n  display: none;\
        \n}\
        \n"
    );
}
