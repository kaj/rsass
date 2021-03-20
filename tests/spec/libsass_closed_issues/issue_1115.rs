//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1115.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n    bar: \"x\\79\";\
            \n    baz: \"#{x}\\79\";\
            \n    bar: \"x\\a\";\
            \n    baz: \"#{x}\\a\";\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: \"xy\";\
        \n  baz: \"xy\";\
        \n  bar: \"x\\a\";\
        \n  baz: \"x\\a\";\
        \n}\
        \n"
    );
}
