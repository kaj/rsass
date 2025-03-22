//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_58.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_58")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("test {\r\
             \n  background: url(/static_loc/img/beta.png);\r\
             \n}"),
        "test {\
         \n  background: url(/static_loc/img/beta.png);\
         \n}\n"
    );
}
