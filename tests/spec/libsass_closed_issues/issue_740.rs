//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_740.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$foo: null;\
            \n$foo: #fff !default;\
            \n$bar: #000;\
            \n$bar: #f00 !default;\
            \n\
            \nfoo {\
            \n  foo: $foo;\
            \n  bar: $bar;\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  foo: #fff;\
        \n  bar: #000;\
        \n}\
        \n"
    );
}
