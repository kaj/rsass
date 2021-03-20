//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_873.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$quoted: \"notification\";\
            \n$unquoted: notification;\
            \n\
            \n@function func($var) {\
            \n  @return $var;\
            \n}\
            \n\
            \nfoo {\
            \n  foo: func(notification);\
            \n  foo: #{notification};\
            \n  foo: #{$quoted};\
            \n  foo: $quoted;\
            \n  foo: #{$unquoted};\
            \n  foo: $unquoted;\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  foo: notification;\
        \n  foo: notification;\
        \n  foo: notification;\
        \n  foo: \"notification\";\
        \n  foo: notification;\
        \n  foo: notification;\
        \n}\
        \n"
    );
}
