//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2429.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2429")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("input[type=url] {\r\
             \n  content: bar\r\
             \n}"),
        "input[type=url] {\
         \n  content: bar;\
         \n}\n"
    );
}
