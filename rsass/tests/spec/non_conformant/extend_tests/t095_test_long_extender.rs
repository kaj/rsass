//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/095_test_long_extender.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("095_test_long_extender")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".foo.bar {a: b}\
             \n.baz.bang {@extend .foo}\n"),
        ".foo.bar, .bar.baz.bang {\
         \n  a: b;\
         \n}\n"
    );
}
