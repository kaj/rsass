//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_864.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass("div { color: desaturate(#999, 50%); }").unwrap(),
        "div {\
        \n  color: #999999;\
        \n}\
        \n"
    );
}
