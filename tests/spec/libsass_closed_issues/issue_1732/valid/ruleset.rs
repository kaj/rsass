//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1732/valid/ruleset.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo { \
            \n  color: green;\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  color: green;\
        \n}\
        \n"
    );
}
