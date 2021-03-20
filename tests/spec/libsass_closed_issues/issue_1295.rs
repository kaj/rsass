//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1295.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  $nothing: null;\
            \n  foo: \"#{$nothing}\' %\' \'#{$nothing}\";\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  foo: \"\' %\' \'\";\
        \n}\
        \n"
    );
}
