//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/069_test_attribute_unification.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("069_test_attribute_unification")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%-a %-a [foo=bar].bar {a: b}\
             \n[foo=bar] {@extend .bar} -a {@extend %-a}\n"),
        "-a -a [foo=bar] {\
         \n  a: b;\
         \n}\n"
    );
}
