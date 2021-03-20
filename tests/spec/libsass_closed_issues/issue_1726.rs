//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1726.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "item {\
            \n    background: #{2px} 2px /*red*/;\
            \n}\
            \n"
        )
        .unwrap(),
        "item {\
        \n  background: 2px 2px;\
        \n}\
        \n"
    );
}
