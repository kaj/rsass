//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1396.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1396")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  foo: foo\"bar\"#{baz};\
             \n  foo: foo\"bar\"baz;\
             \n}\n"),
        "foo {\
         \n  foo: foo \"bar\" baz;\
         \n  foo: foo \"bar\" baz;\
         \n}\n"
    );
}
