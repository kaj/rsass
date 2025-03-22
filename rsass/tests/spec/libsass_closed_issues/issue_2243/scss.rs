//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2243/scss.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("scss")
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
