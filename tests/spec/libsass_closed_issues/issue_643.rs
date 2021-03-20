//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_643.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$map: (foo: bar, bar: baz);\
            \n\
            \nfoo {\
            \n  a: nth($map, 2);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: bar baz;\
        \n}\
        \n"
    );
}
