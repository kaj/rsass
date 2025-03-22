//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/045_test_parent_selectors.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("045_test_parent_selectors")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  &:hover {a: b}\
             \n  bar &.baz {c: d}}\n"),
        "foo:hover {\
         \n  a: b;\
         \n}\
         \nbar foo.baz {\
         \n  c: d;\
         \n}\n"
    );
}
