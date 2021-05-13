//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2429.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
