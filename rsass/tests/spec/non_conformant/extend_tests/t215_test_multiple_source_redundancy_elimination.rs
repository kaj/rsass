//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/215_test_multiple_source_redundancy_elimination.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("215_test_multiple_source_redundancy_elimination")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%default-color {color: red}\
             \n%alt-color {color: green}\n\
             \n%default-style {\
             \n@extend %default-color;\
             \n&:hover {@extend %alt-color}\
             \n&:active {@extend %default-color}\
             \n}\n\
             \n.test-case {@extend %default-style}\n"),
        ".test-case:active, .test-case {\
         \n  color: red;\
         \n}\
         \n.test-case:hover {\
         \n  color: green;\
         \n}\n"
    );
}
