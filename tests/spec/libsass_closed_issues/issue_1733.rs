//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1733.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  a: #ff6600;\
            \n  b: #ff6600\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: #ff6600;\
        \n  b: #ff6600;\
        \n}\
        \n"
    );
}
