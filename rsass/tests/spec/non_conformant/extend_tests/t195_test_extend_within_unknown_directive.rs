//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/195_test_extend_within_unknown_directive.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("195_test_extend_within_unknown_directive")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("@flooblehoof {\
             \n.foo {a: b}\
             \n.bar {@extend .foo}\
             \n}\n"),
        "@flooblehoof {\
         \n  .foo, .bar {\
         \n    a: b;\
         \n  }\
         \n}\n"
    );
}
