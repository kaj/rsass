//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/51_trailing_commas_in_list.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("51_trailing_commas_in_list")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n$mylist: (alpha, beta, gamma, );\
             \n$my-single-item-list: (alpha,);\
             \n.test { \
             \n  out1: list.length($mylist);\
             \n  blah: meta.type-of(list.nth($mylist,3));\
             \n  out: list.length($my-single-item-list); \
             \n}"),
        ".test {\
         \n  out1: 3;\
         \n  blah: string;\
         \n  out: 1;\
         \n}\n"
    );
}
