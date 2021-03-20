//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_817.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  foo: url(\'foo/bar.baz\');\
            \n  foo: url(\"foo/bar.baz\");\
            \n  foo: url(foo/bar.baz);\
            \n  foo: foo(\'foo/bar.baz\', \"bar\", 55);\
            \n  foo: foo(\"foo/bar.baz\", \'bar\', 55);\
            \n  foo: foo(\"foo/bar.baz\", bar, 55); }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  foo: url(\"foo/bar.baz\");\
        \n  foo: url(\"foo/bar.baz\");\
        \n  foo: url(foo/bar.baz);\
        \n  foo: foo(\"foo/bar.baz\", \"bar\", 55);\
        \n  foo: foo(\"foo/bar.baz\", \"bar\", 55);\
        \n  foo: foo(\"foo/bar.baz\", bar, 55);\
        \n}\
        \n"
    );
}
