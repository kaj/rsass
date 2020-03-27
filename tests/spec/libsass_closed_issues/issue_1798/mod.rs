//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1798"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_1798/1.hrx"
#[test]
fn t1() {
    assert_eq!(
        rsass(
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

// From "sass-spec/spec/libsass-closed-issues/issue_1798/2.hrx"
#[test]
fn t2() {
    assert_eq!(
        rsass(
            "a /*#{#{*/ {\
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
