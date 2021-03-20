//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1086.hrx"

#[test]
fn test() {
    assert_eq!(crate::rsass("$map: (-1px: 12);").unwrap(), "");
}
