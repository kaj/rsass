//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1647/selectors.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$map: (foo: \'b\', bar: c);\
            \n$list: (\'d\', e);\
            \n\
            \na {\
            \n  #{map-get($map, foo)} & {\
            \n      foo: bar;\
            \n  }\
            \n  #{map-get($map, bar)} & {\
            \n      foo: bar;\
            \n  }\
            \n\
            \n  #{nth($list, 1)} & {\
            \n      foo: bar;\
            \n  }\
            \n\
            \n  #{nth($list, 2)} & {\
            \n      foo: bar;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "b a {\
        \n  foo: bar;\
        \n}\
        \nc a {\
        \n  foo: bar;\
        \n}\
        \nd a {\
        \n  foo: bar;\
        \n}\
        \ne a {\
        \n  foo: bar;\
        \n}\
        \n"
    );
}
