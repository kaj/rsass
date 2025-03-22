//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2120.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2120")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@import url(//xyz.cöm/ürl)"),
        "@charset \"UTF-8\";\
         \n@import url(//xyz.cöm/ürl);\n"
    );
}
