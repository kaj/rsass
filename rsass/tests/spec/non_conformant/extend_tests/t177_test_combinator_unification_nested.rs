//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/177_test_combinator_unification_nested.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("177_test_combinator_unification_nested")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".a > .b + x {a: b}\
             \n.c > y {@extend x}\n"),
        ".a > .b + x, .a.c > .b + y {\
         \n  a: b;\
         \n}\n"
    );
}
