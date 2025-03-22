//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1163.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1163")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  content: (((92px * 12) / 13px) * 1em) + 22em;\
             \n}"),
        "div {\
         \n  content: 106.9230769231em;\
         \n}\n"
    );
}
