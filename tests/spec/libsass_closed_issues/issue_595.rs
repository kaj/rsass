//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_595.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {\
             \n    color: red;\
             \n};\n"),
        "a {\
         \n  color: red;\
         \n}\n"
    );
}
