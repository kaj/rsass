//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_394.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$list1: alpha beta gamma;\
            \n$list2: one two three;\
            \n\
            \n$map: (alpha: one, beta: two, gamma: three);\
            \n\
            \n.ma-list {\
            \n  @each $item1, $item2 in zip($list1, $list2) {\
            \n    #{$item1}: $item2;\
            \n  }\
            \n}\
            \n\
            \n.ma-map {\
            \n  @each $key, $value in $map {\
            \n    #{$key}: $value;\
            \n  }\
            \n}"
        )
        .unwrap(),
        ".ma-list {\
        \n  alpha: one;\
        \n  beta: two;\
        \n  gamma: three;\
        \n}\
        \n.ma-map {\
        \n  alpha: one;\
        \n  beta: two;\
        \n  gamma: three;\
        \n}\
        \n"
    );
}
