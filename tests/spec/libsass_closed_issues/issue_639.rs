//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_639.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$quoted_list: \"foo\", \"bar\", \"baz\";\
             \n$unquoted_list: foo, bar, baz;\n\
             \nfoo {\
             \n  foo: #{foo, bar, baz};\
             \n  foo: #{\"foo\", \"bar\", \"baz\"};\
             \n  foo: #{$quoted_list};\
             \n  foo: #{$unquoted_list};\
             \n}\n"),
        "foo {\
         \n  foo: foo, bar, baz;\
         \n  foo: foo, bar, baz;\
         \n  foo: foo, bar, baz;\
         \n  foo: foo, bar, baz;\
         \n}\n"
    );
}
