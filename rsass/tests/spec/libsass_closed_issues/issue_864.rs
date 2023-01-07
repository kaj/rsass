//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_864.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_864")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div { color: desaturate(#999, 50%); }"),
        "div {\
         \n  color: #999999;\
         \n}\n"
    );
}
