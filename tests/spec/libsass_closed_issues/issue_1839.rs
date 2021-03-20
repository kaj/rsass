//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1839.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass("@custom-media --large-viewport (min-width: 1001px);")
            .unwrap(),
        "@custom-media --large-viewport (min-width: 1001px);\
        \n"
    );
}
