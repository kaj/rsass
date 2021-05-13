//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/parent_selectors.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
