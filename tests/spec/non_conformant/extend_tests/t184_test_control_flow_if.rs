//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/184_test_control_flow_if.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".true  { color: green; }\
            \n.false { color: red;   }\
            \n.also-true {\
            \n@if true { @extend .true;  }\
            \n@else    { @extend .false; }\
            \n}\
            \n.also-false {\
            \n@if false { @extend .true;  }\
            \n@else     { @extend .false; }\
            \n}\
            \n"
        )
        .unwrap(),
        ".true, .also-true {\
        \n  color: green;\
        \n}\
        \n.false, .also-false {\
        \n  color: red;\
        \n}\
        \n"
    );
}
