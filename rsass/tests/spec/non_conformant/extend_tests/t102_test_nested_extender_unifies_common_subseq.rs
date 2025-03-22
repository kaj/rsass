//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/102_test_nested_extender_unifies_common_subseq.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("102_test_nested_extender_unifies_common_subseq")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(
            ".a .x .b .y .foo {a: b}\
             \n.a .n .b .m bar {@extend .foo}\n"
        ),
        ".a .x .b .y .foo, .a .x .n .b .y .m bar, .a .n .x .b .y .m bar, .a .x .n .b .m .y bar, .a .n .x .b .m .y bar {\
         \n  a: b;\
         \n}\n"
    );
}
