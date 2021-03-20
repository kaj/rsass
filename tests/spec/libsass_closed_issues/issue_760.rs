//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_760.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  quoted: str-slice(\"abcd\", 1, 0);\
            \n  unquoted: str-slice(abcd, 1, 0);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  quoted: \"\";\
        \n}\
        \n"
    );
}
