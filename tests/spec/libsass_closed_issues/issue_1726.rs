//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1726.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("item {\
             \n    background: #{2px} 2px /*red*/;\
             \n}\n"),
        "item {\
         \n  background: 2px 2px;\
         \n}\n"
    );
}
