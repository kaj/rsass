//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/162_test_combinator_unification_double_plus.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("162_test_combinator_unification_double_plus")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".a + x {a: b}\
             \n.b + y {@extend x}\n"),
        ".a + x, .a.b + y {\
         \n  a: b;\
         \n}\n"
    );
}
