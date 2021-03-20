//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_890.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n  border: {\
            \n    right: 10px solid /*here is a comment*/;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  border-right: 10px solid;\
        \n}\
        \n"
    );
}
