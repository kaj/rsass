//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1162.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  content: #{0/0} a;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  content: 0/0 a;\
        \n}\
        \n"
    );
}
