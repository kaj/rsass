//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1741.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".header {\r\
            \n  .nav-text-link:not(&.popover-link) {\r\
            \n    margin: 10px;\r\
            \n  }\r\
            \n}"
        )
        .unwrap(),
        ".nav-text-link:not(.header.popover-link) {\
        \n  margin: 10px;\
        \n}\
        \n"
    );
}
