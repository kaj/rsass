//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/046_test_parent_selector_with_subject.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  bar &.baz! .bip {a: b}}\n\
             \nfoo bar {\
             \n  bar &.baz! .bip {c: d}}\n"),
        "bar foo.baz! .bip {\
         \n  a: b;\
         \n}\
         \nbar foo bar.baz! .bip {\
         \n  c: d;\
         \n}\n"
    );
}
