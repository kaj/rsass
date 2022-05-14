//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/184_test_control_flow_if.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("184_test_control_flow_if")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".true  { color: green; }\
             \n.false { color: red;   }\
             \n.also-true {\
             \n@if true { @extend .true;  }\
             \n@else    { @extend .false; }\
             \n}\
             \n.also-false {\
             \n@if false { @extend .true;  }\
             \n@else     { @extend .false; }\
             \n}\n"),
        ".true, .also-true {\
         \n  color: green;\
         \n}\
         \n.false, .also-false {\
         \n  color: red;\
         \n}\n"
    );
}
