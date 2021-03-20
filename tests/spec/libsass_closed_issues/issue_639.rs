//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_639.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$quoted_list: \"foo\", \"bar\", \"baz\";\
            \n$unquoted_list: foo, bar, baz;\
            \n\
            \nfoo {\
            \n  foo: #{foo, bar, baz};\
            \n  foo: #{\"foo\", \"bar\", \"baz\"};\
            \n  foo: #{$quoted_list};\
            \n  foo: #{$unquoted_list};\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  foo: foo, bar, baz;\
        \n  foo: foo, bar, baz;\
        \n  foo: foo, bar, baz;\
        \n  foo: foo, bar, baz;\
        \n}\
        \n"
    );
}
