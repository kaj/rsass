//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1527/selector/last.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("last")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("foo & {\
             \n  bar: baz;\
             \n}\n"),
        "foo & {\
         \n  bar: baz;\
         \n}\n"
    );
}
