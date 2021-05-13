//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/118_test_parent_selector_with_parent_and_subject.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("$subject: \"!\";\
             \nfoo {\
             \n  bar &.baz#{$subject} .bip {c: d}}\n"),
        "bar foo.baz! .bip {\
         \n  c: d;\
         \n}\n"
    );
}
