//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2625.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
