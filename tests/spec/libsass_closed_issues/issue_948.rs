//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_948.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo { bar: 10 * 5#{px}; }"),
        "foo {\
         \n  bar: 50 px;\
         \n}\n"
    );
}
