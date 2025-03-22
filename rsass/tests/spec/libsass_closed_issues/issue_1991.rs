//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1991.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1991")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$tests: (\
             \n  0: \'foo\',\
             \n  false: \'bar\',\
             \n);"),
        ""
    );
}
