//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1732/valid/propset.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo { \
            \n  border: {\
            \n    width: 1px;\
            \n    color: green;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  border-width: 1px;\
        \n  border-color: green;\
        \n}\
        \n"
    );
}
