//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2243"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_2243/sass.hrx"

// From "sass-spec/spec/libsass-closed-issues/issue_2243/scss.hrx"
#[test]
fn scss() {
    assert_eq!(
        rsass(
            "@namespace url(\"http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul\")\
            \n"
        )
        .unwrap(),
        "@namespace url(\"http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul\");\
        \n"
    );
}
