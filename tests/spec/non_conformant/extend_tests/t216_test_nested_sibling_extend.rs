//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/216_test_nested_sibling_extend.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".foo {@extend .bar}\n\
             \n.parent {\
             \n.bar {\
             \n  width: 2000px;\
             \n}\
             \n.foo {\
             \n  @extend .bar\
             \n}\
             \n}\n"),
        ".parent .bar, .parent .foo {\
         \n  width: 2000px;\
         \n}\n"
    );
}
