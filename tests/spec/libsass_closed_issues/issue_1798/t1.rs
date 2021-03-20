//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1798/1.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "a /*#{\"}*/ {\
            \n  margin: 2px;\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  margin: 2px;\
        \n}\
        \n"
    );
}
