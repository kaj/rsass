//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_492.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$map: (\
            \n  foo: bar,\
            \n  baz: monkey,\
            \n);\
            \n\
            \n.css {\
            \n  @each $key, $value in $map {\
            \n    #{$key}: $value;\
            \n  }\
            \n}\
            \n\
            \n$list: one two, three four five, six seven;\
            \n\
            \n.list {\
            \n  @each $foo, $bar, $baz in $list {\
            \n    #{$foo}: $bar $baz;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".css {\
        \n  foo: bar;\
        \n  baz: monkey;\
        \n}\
        \n.list {\
        \n  one: two;\
        \n  three: four five;\
        \n  six: seven;\
        \n}\
        \n"
    );
}
