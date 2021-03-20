//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1732/valid/mixin-call.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin bar() {\
            \n  @content;\
            \n}\
            \n\
            \nfoo {\
            \n  @include bar {\
            \n    color: blue;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  color: blue;\
        \n}\
        \n"
    );
}
