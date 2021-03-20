//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1722.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$score: (item-height: 1.12em);\
            \n.test {\
            \n    background-position: 0 -#{map-get($score, item-height)};\
            \n}\
            \n\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  background-position: 0 -1.12em;\
        \n}\
        \n"
    );
}
