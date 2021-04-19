//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1163.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  content: (((92px * 12) / 13px) * 1em) + 22em;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  content: 106.9230769231em;\
        \n}\
        \n"
    );
}
