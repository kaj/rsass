//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1269.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function push($list, $items...) {\
            \n  @return join($list, $items, $separator: auto);\
            \n}\
            \n\
            \n.test {\
            \n  $list: push(1 2 3, 4, 5);\
            \n  list: inspect($list);\
            \n  value: nth($list, 4);\
            \n}"
        )
        .unwrap(),
        ".test {\
        \n  list: 1 2 3 4 5;\
        \n  value: 4;\
        \n}\
        \n"
    );
}
