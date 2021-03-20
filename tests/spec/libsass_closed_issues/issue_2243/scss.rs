//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2243/scss.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@namespace url(\"http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul\")\
            \n"
        )
        .unwrap(),
        "@namespace url(\"http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul\");\
        \n"
    );
}
