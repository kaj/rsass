//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1208.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  &.bar, /* */\
            \n  &.baz {\
            \n    color: red;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "foo.bar, foo.baz {\
        \n  color: red;\
        \n}\
        \n"
    );
}
