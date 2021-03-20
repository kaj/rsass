//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_2096.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin foo() {\
            \n  @import \"https://foo\";\
            \n}\
            \n@include foo;\
            \n"
        )
        .unwrap(),
        "@import \"https://foo\";\
        \n"
    );
}
