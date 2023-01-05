//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/221_test_partially_failed_extend.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("221_test_partially_failed_extend")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("test { @extend .rc; }\
             \n.rc {color: white;}\
             \n.prices span.pill span.rc {color: red;}\n"),
        ".rc, test {\
         \n  color: white;\
         \n}\
         \n.prices span.pill span.rc {\
         \n  color: red;\
         \n}\n"
    );
}
