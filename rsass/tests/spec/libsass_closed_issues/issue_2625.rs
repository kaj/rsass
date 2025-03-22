//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2625.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2625")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("something\\:{ padding: 2px; }\n"),
        "something\\: {\
         \n  padding: 2px;\
         \n}\n"
    );
}
