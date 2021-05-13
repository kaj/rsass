//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1162.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  content: #{0/0} a;\
             \n}"),
        "div {\
         \n  content: 0/0 a;\
         \n}\n"
    );
}
