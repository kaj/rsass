//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1240.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$var: 1;\
            \n$list: 2, 3;\
            \n$new-list: append($var, $list);\
            \n$nested-list: $var $list;\
            \n@debug($var);\
            \n@debug($list);\
            \n@debug($new-list);\
            \n@debug($nested-list);\
            \ndiv {\
            \n a: $var;\
            \n a: $list;\
            \n a: $new-list;\
            \n a: $nested-list;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  a: 1;\
        \n  a: 2, 3;\
        \n  a: 1 2, 3;\
        \n  a: 1 2, 3;\
        \n}\
        \n"
    );
}
