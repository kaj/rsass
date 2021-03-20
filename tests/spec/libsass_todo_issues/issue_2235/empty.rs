//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_2235/empty.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
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
