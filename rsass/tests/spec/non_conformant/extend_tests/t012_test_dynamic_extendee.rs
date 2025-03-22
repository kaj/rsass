//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/012_test_dynamic_extendee.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("012_test_dynamic_extendee")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".foo {a: b}\
             \n.bar {@extend #{\".foo\"}}\n"),
        ".foo, .bar {\
         \n  a: b;\
         \n}\n"
    );
}
