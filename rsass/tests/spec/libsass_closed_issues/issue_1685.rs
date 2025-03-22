//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1685.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1685")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function foo($x, $y...) { @return null }\n\
             \na {\
             \n  b: foo(1 2 3...);\
             \n}"),
        ""
    );
}
