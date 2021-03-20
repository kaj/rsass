//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1765.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  bar: 20px /* height */ + 2*5px /* padding */ + 2*1px /*border*/;\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: 32px;\
        \n}\
        \n"
    );
}
