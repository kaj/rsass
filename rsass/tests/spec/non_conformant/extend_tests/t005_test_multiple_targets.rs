//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/005_test_multiple_targets.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("005_test_multiple_targets")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".foo {a: b}\
             \n.bar {@extend .foo}\
             \n.blip .foo {c: d}\n"),
        ".foo, .bar {\
         \n  a: b;\
         \n}\
         \n.blip .foo, .blip .bar {\
         \n  c: d;\
         \n}\n"
    );
}
