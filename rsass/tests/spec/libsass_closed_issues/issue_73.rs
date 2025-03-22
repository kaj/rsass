//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_73.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_73")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin box-shadow($shadow...) { \r\
             \n  -webkit-box-shadow: $shadow;\r\
             \n     -moz-box-shadow: $shadow;\r\
             \n          box-shadow: $shadow;\r\
             \n}"),
        ""
    );
}
