//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/51_trailing_commas_in_list.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$mylist: (alpha, beta, gamma, );\
            \n$my-single-item-list: (alpha,);\
            \n.test { \
            \n  out1: length($mylist);\
            \n  blah: type-of(nth($mylist,3));\
            \n  out: length($my-single-item-list); \
            \n}"
        )
        .unwrap(),
        ".test {\
        \n  out1: 3;\
        \n  blah: string;\
        \n  out: 1;\
        \n}\
        \n"
    );
}
