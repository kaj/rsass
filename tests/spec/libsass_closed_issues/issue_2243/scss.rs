//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2243/scss.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@namespace url(\"http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul\")\n"
        ),
        "@namespace url(\"http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul\");\n"
    );
}
