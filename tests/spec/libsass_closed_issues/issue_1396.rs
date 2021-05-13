//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1396.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
