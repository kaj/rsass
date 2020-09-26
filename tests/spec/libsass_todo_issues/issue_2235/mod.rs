//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_2235"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-todo-issues/issue_2235/empty.hrx"
#[test]
fn empty() {
    assert_eq!(
        rsass(
            "@media all and (min-width: 100px) {\
            \n  @import \"https://example.org\";\
            \n}\
            \n"
        )
        .unwrap(),
        "@media all and (min-width: 100px) {\
        \n  @import \"https://example.org\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-todo-issues/issue_2235/not-empty.hrx"
#[test]
fn not_empty() {
    assert_eq!(
        rsass(
            "@media all and (min-width: 100px) {\
            \n  a { b: c }\
            \n  @import \"https://example.org\";\
            \n}\
            \n"
        )
        .unwrap(),
        "@media all and (min-width: 100px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n  @import \"https://example.org\";\
        \n}\
        \n"
    );
}
