//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1074.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$i: 1;\
            \n.foo#{-$i} { a:b }\
            \n.foo-#{$i} { a:b }\
            \n.foo#{-1} { a:b }\
            \n.foo-#{1} { a:b }\
            \n"
        )
        .unwrap(),
        ".foo-1 {\
        \n  a: b;\
        \n}\
        \n.foo-1 {\
        \n  a: b;\
        \n}\
        \n.foo-1 {\
        \n  a: b;\
        \n}\
        \n.foo-1 {\
        \n  a: b;\
        \n}\
        \n"
    );
}
