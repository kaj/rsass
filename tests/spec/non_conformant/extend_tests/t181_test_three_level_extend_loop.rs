//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/181_test_three_level_extend_loop.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("181_test_three_level_extend_loop")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".foo {a: b; @extend .bar}\
             \n.bar {c: d; @extend .baz}\
             \n.baz {e: f; @extend .foo}\n"),
        ".foo, .baz, .bar {\
         \n  a: b;\
         \n}\
         \n.bar, .foo, .baz {\
         \n  c: d;\
         \n}\
         \n.baz, .bar, .foo {\
         \n  e: f;\
         \n}\n"
    );
}
