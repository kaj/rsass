//! Tests auto-converted from "sass-spec/spec/css/plain/import/url.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("url")
}

#[test]
#[ignore] // unexepected error
fn escaped() {
    assert_eq!(
        runner().ok("@import \\75 rl(http://example.org);\n"),
        "@import url(http://example.org);\n"
    );
}
